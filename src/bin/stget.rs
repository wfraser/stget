#[macro_use] extern crate log;

use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use byteorder::{ByteOrder, NetworkEndian};
use stget::syncthing_proto as proto;

fn main() {
    env_logger::init();

    let args = clap::Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("experimental Syncthing file retrieval program")
        .arg(clap::Arg::new("address")
                .help("Address of the remote host. Port 22000 is used if unspecified.")
                .required(true)
                .index(1))
        .arg(clap::Arg::new("device_id")
                .help("Device ID of the remote host.")
                .required(true)
                .index(2))
        .arg(clap::Arg::new("path")
                .help("File path to fetch.")
                .index(3))
        .arg(clap::Arg::new("list")
                .short('l')
                .long("list")
                .action(clap::ArgAction::SetTrue)
                .help("List all files on the remote end."))
        .arg(clap::Arg::new("destination")
                .short('d')
                .long("dest")
                .help("destination path for downloaded file(s)"))
        .group(clap::ArgGroup::new("path_or_list")
                .args(["path", "list"])
                .required(true))
        .get_matches();

    let host_and_port = match args.get_one::<String>("address").unwrap() {
        host if host.contains(':') => host.to_owned(),
        host => {
            debug!("no port specified; assuming 22000");
            format!("{}:22000", host)
        }
    };

    let device_id = args.get_one::<String>("device_id").unwrap();
    if device_id.len() != 63 {
        eprintln!("Device ID should be 63 characters long, not {}", device_id.len());
        std::process::exit(1);
    }

    let remote_cert_hash = stget::util::hash_from_device_id(device_id);

    /*
    // FIXME(wfraser) remove this
    let (host_and_port, device_id) =
        ("127.0.0.1:22000", "JDF55R5-QQJBXUN-QQPSVFT-HFCAV6J-7NSVM7I-2KBA7PI-4MGOAIR-FA3I4AH");
    */

    // TODO(wfraser) base path should be in $XDG_CONFIG_DIRS or something
    let base_path = std::env::current_dir().unwrap_or_else(|e| {
        panic!("unable to get working directory: {}", e);
    });

    // TODO(wfraser) make this configurable
    let cert_path = base_path.join("cert").join("cert.pem");
    let key_path = base_path.join("cert").join("private.pem");

    let cert = stget::certificate::read_cert_file_pem(&cert_path).unwrap_or_else(|e| {
        eprintln!("Unable to load certificate {:?}: {}", cert_path, e);
        eprintln!("Did you remember to generate a client certificate?");
        std::process::exit(1);
    });
    let key = stget::certificate::read_key_file_pem(&key_path).unwrap_or_else(|e| {
        eprintln!("Unable to load private key {:?}: {}", key_path, e);
        eprintln!("Did you remember to generate a client certificate?");
        std::process::exit(1);
    });

    let mut session = stget::session::SessionBuilder {
        remote_host_and_port: host_and_port,
        remote_device_id: device_id.to_string(),
        local_device_name: None,
        client_cert: cert,
        private_key: key,
    }.connect().expect("Failed to create TLS session");

    session.write_hello().unwrap();

    let mut program_state = ProgramState {
        remote_cert_hash,
        folders_by_id: HashMap::new(),
        mode: if args.get_flag("list") {
            Mode::List
        } else {
            let path = args.get_one::<String>("path").unwrap().to_owned();
            if !path.contains('/') {
                panic!("To fetch an entire folder, append a '/' to the path.");
            }
            Mode::Fetch(path)
        },
        destination: args.get_one::<String>("destination").map(|s| s.as_str()).unwrap_or(".").to_owned(),
        protocol_state: Some(State::ExpectHello),
    };

    let mut data = vec![];
    while session.complete_io().is_ok() {
        match session.read_to_end(&mut data) {
            Ok(n) => {
                debug!("read {}", n);

                if n > 0 {
                    let new_state = process_network_data(
                        &mut program_state,
                        &mut session,
                        &mut data);
                    if let State::Done = new_state {
                        break;
                    } else {
                        program_state.protocol_state = Some(new_state);
                    }
                }
            }
            Err(e) => {
                if let Some(e) = e.downcast_ref::<std::io::Error>() {
                    if e.kind() == std::io::ErrorKind::ConnectionAborted {
                        eprintln!("Connection closed.");
                    } else {
                        eprintln!("Read error: {}", e);
                    }
                } else {
                    eprintln!("Unknown error: {}", e);
                }
                break;
            }
        }
    }

    match program_state.protocol_state {
        Some(State::ExpectHello) | Some(State::ExpectClusterConfig) => {
            eprintln!("Remote declined to talk with us.");
            let (_len, remote_hello): (usize, proto::Hello) =
                stget::session::Session::read_hello(&data).unwrap_or_else(|e| {
                    panic!("error reading remote hello: {}", e);
                });
            eprintln!("Remote is \"{}\", running {} {}",
                      remote_hello.device_name,
                      remote_hello.client_name,
                      remote_hello.client_version);
        }
        _ => ()
    }
}

fn process_network_data(
    program: &mut ProgramState,
    session: &mut stget::session::Session,
    data: &mut Vec<u8>,
) -> State {
    match program.protocol_state.take().unwrap() {
        State::ExpectHello => {
            debug!("got hello");
            program.handle_hello(data)
        },
        State::ExpectClusterConfig => {
            debug!("got remote cluster config");
            program.handle_cluster_config(data, session)
        },
        State::IndexOrBlocks(index_recv_state, fetch_state) => {
            program.handle_index_or_response(
                data,
                session,
                index_recv_state,
                fetch_state)
        }
        State::Done => panic!("bad state"),
    }
}

#[derive(Debug)]
struct ProgramState {
    remote_cert_hash: Vec<u8>,
    folders_by_id: HashMap<String, FolderInfo>,
    mode: Mode,
    destination: String,
    protocol_state: Option<State>,
}

#[derive(Debug)]
enum Mode {
    List,
    Fetch(String),
}

#[derive(Debug)]
enum State {
    Done,
    ExpectHello,
    ExpectClusterConfig,
    IndexOrBlocks(IndexRecvState, Option<BlockFetchState>),
}

#[derive(Debug)]
struct FolderInfo {
    label: String,
    max_remote_seq: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IndexRecvState {
    index_n: usize,
}

#[derive(Debug)]
struct BlockFetchState {
    request_map: HashMap<i32, FileFetchState>,
}

#[derive(Debug)]
struct FileFetchState {
    file: File,
    size: u64,
    read_bytes: u64,
    all_blocks: Vec<proto::BlockInfo>,
    current_outstanding_idx: usize,
    folder_id: String,
    path: String,
}

impl ProgramState {
    pub fn handle_hello(&self, data: &mut Vec<u8>) -> State {
        let (len, remote_hello): (usize, proto::Hello) =
            stget::session::Session::read_hello(data).unwrap_or_else(|e| {
                panic!("error reading remote hello: {}", e);
            });
        eprintln!("Remote is \"{}\", running {} {}",
                remote_hello.device_name,
                remote_hello.client_name,
                remote_hello.client_version);
        data.drain(0..len);

        // Wait to send cluster config until we read the remote one.
        State::ExpectClusterConfig
    }

    pub fn handle_cluster_config(
        &mut self, data: &mut Vec<u8>,
        session: &mut stget::session::Session,
    ) -> State {
        let (len, msgtype, message) = stget::session::Session::read_message(data)
            .unwrap_or_else(|e| {
                panic!("Error reading remote cluster config: {}", e);
            });
        data.drain(0..len);

        let remote_cluster_config: &proto::ClusterConfig = match msgtype {
            proto::MessageType::CLUSTER_CONFIG => message.as_any().downcast_ref().unwrap(),
            other => {
                eprintln!("unexpected message type {:?}; wanted CLUSTER_CONFIG", other);
                return State::Done;
            }
        };

        debug!("remote cluster config: {:#?}", remote_cluster_config);

        let mut cluster_config = proto::ClusterConfig::new();

        for folder in &remote_cluster_config.folders {
            for device in &folder.devices {
                let device_cert_hash: &[u8] = &device.id;
                if device_cert_hash == self.remote_cert_hash.as_slice() {
                    self.folders_by_id.insert(
                        folder.id.clone(),
                        FolderInfo {
                            label: folder.label.clone(),
                            max_remote_seq: device.max_sequence,
                        });
                }
            }
        }

        match self.mode {
            Mode::List => {
                // make a cluster config with all the folders in the remote
                for remote_folder in &remote_cluster_config.folders {
                    let mut folder = proto::Folder::new();
                    folder.id = remote_folder.id.clone();
                    folder.label = remote_folder.label.clone();
                    folder.read_only = true;
                    folder.ignore_permissions = true;
                    folder.ignore_delete = true;
                    folder.disable_temp_indexes = true;
                    cluster_config.folders.push(folder);
                }
            },
            Mode::Fetch(ref path) => {
                let folder_name = path.split('/').next().unwrap();

                let mut folder_id = None;
                for folder in &remote_cluster_config.folders {
                    if folder.label == folder_name {
                        folder_id = Some(folder.id.clone());
                        break;
                    }
                }
                if folder_id.is_none() {
                    eprintln!("The remote computer is not offering a folder with the specified name (\"{}\").", folder_name);
                    eprintln!("it offered:");
                    for folder in &remote_cluster_config.folders {
                        eprintln!("    {} ({})", folder.label, folder.id);
                    }
                    return State::Done;
                }

                let mut folder = proto::Folder::new();
                folder.id = folder_id.unwrap();
                folder.label = folder_name.to_owned();
                folder.read_only = true;
                folder.ignore_permissions = true;
                folder.ignore_delete = true;
                folder.disable_temp_indexes = true;
                cluster_config.folders.push(folder);
            }
        }

        debug!("sending cluster config");
        session.write_message(
                &cluster_config,
                proto::MessageType::CLUSTER_CONFIG)
            .unwrap_or_else(|e| {
                panic!("error sending our cluster config: {}", e);
            });

        eprintln!("receiving folder index");
        let index_recv_state = IndexRecvState {
            index_n: 0,
        };
        State::IndexOrBlocks(index_recv_state, None)
    }

    pub fn handle_index_or_response(
        &mut self,
        data: &mut Vec<u8>,
        session: &mut stget::session::Session,
        mut index_recv_state: IndexRecvState,
        mut fetch_state: Option<BlockFetchState>,
    ) -> State {
        let header_len = NetworkEndian::read_u16(&data[0..2]) as usize;
        let body_len = NetworkEndian::read_u32(&data[
            2 + header_len
                .. 2 + header_len + 4]) as usize;
        let data_len = 2 + header_len + 4 + body_len;
        if data.len() < data_len {
            debug!("not enough data; reading more (need {}, have {})",
                    data_len, data.len());
            return State::IndexOrBlocks(index_recv_state, fetch_state);
        }

        let (input_pos, msgtype, mut message) = stget::session::Session::read_message(data)
            .unwrap_or_else(|e| {
                panic!("Error reading remote message: {}", e);
            });
        debug!("{} bytes read", input_pos);
        assert_eq!(data_len, input_pos);

        match msgtype {
            proto::MessageType::INDEX => {
                let index: &proto::Index = message.as_any().downcast_ref().unwrap();
                self.handle_index(index, session, &mut index_recv_state, &mut fetch_state);
            },
            proto::MessageType::INDEX_UPDATE => {
                // Horrible hack relying on the fact that Index and IndexUpdate are the same
                let index: &proto::Index = unsafe {
                    &*(message.as_any().downcast_ref::<proto::IndexUpdate>().unwrap()
                        as *const proto::IndexUpdate
                        as *const proto::Index)
                };
                self.handle_index(index, session, &mut index_recv_state, &mut fetch_state);
            },
            proto::MessageType::PING => {
                debug!("got a ping message");
                return State::IndexOrBlocks(index_recv_state, fetch_state);
            },
            proto::MessageType::CLOSE => {
                let close: &proto::Close = message.as_any().downcast_ref().unwrap();
                debug!("got a close message: {:?}", close);
                return State::Done;
            },
            proto::MessageType::RESPONSE if fetch_state.is_some() => {
                debug!("got a RESPONSE message");
                let msg: &mut proto::Response = message.as_any_mut().downcast_mut().unwrap();

                let fetch_state: &mut BlockFetchState = fetch_state.as_mut().unwrap();

                let file_state = fetch_state.request_map
                    .remove(&msg.id).expect("unexpected message ID");

                match self.handle_response(msg, session, file_state) {
                    Some((req_id, new_file_state)) => {
                        fetch_state.request_map.insert(req_id, new_file_state);
                    }
                    None => {
                        if fetch_state.request_map.is_empty() {
                            return State::Done;
                        }
                    }
                }
            },
            other => {
                eprintln!("got an unexpected message type: {:?}", other);
                return State::Done;
            }
        };

        data.drain(0..data_len);

        if let (&Mode::Fetch(_), true) = (
                &self.mode,
                index_recv_state.index_n != 0 && fetch_state.is_none()
        ) {
            eprintln!("No matching file was found in the directory index.");
            State::Done
        } else if index_recv_state.index_n == self.folders_by_id.len() {
            // all done :)
            State::Done
        } else {
            State::IndexOrBlocks(index_recv_state, fetch_state)
        }
    }

    // Return the destination path for the given file if it matches the check pattern, or None if it
    // doesn't.
    fn dest_path(&self, file_path: &str, check_path: &str, folder: &str) -> Option<PathBuf> {

        let file_part = if check_path.is_empty() {
            // Degenerate case (whole folder): folder name plus full file path.
            return Some(Path::new(&self.destination).join(folder).join(file_path));
        } else if check_path.ends_with('/')
                && file_path.starts_with(check_path) {
            // Folder match; start from the last component of the check path.
            let prefix_len = match check_path.rsplitn(3, '/').nth(2) {
                Some(prefix) => prefix.len() + 1,
                None => 0,
            };
            Path::new(&file_path[prefix_len..])
        } else if file_path == check_path {
            // Exact match; just use the file name.
            Path::new(Path::new(file_path).file_name().unwrap())
        } else {
            return None;
        };

        Some(Path::new(&self.destination)
            .join(file_part))
    }

    fn handle_index(
        &self,
        index: &proto::Index,
        session: &mut stget::session::Session,
        index_recv_state: &mut IndexRecvState,
        fetch_state: &mut Option<BlockFetchState>,
    ) {
        debug!("remote index: {:#?}", index);

        let folder_info = &self.folders_by_id[&index.folder];

        for file in &index.files {
            if file.deleted {
                continue;
            }
            if file.type_.unwrap() == proto::FileInfoType::DIRECTORY {
                if let Mode::Fetch(ref check_path) = self.mode {
                    if !check_path.ends_with('/')
                         && check_path[folder_info.label.len() + 1 ..] == file.name
                    {
                        panic!("Cannot fetch a directory entry. To recursively fetch a whole \
                                directory, append a '/' to the path.");
                    }
                }
                continue;
            }

            let display_path = format!("{}/{}", folder_info.label, file.name);
            match self.mode {
                Mode::List => {
                    println!("{}", display_path);
                }
                Mode::Fetch(ref check_path) => {
                    let dest_path = match self.dest_path(
                            &file.name,
                            &check_path[folder_info.label.len() + 1 ..],
                            &folder_info.label) {
                        Some(p) => p,
                        None => continue
                    };

                    debug!("found matching file: {:?}", display_path);
                    eprintln!("requesting file: {:?}", display_path);
                    debug!("destination path: {:?}", dest_path);

                    std::fs::create_dir_all(dest_path.parent().unwrap()).unwrap();
                    let fs_file = File::create(dest_path).unwrap();

                    let req_id = session.write_block_request(
                        index.folder.clone(),
                        file.name.to_owned(),
                        file.blocks[0].offset,
                        file.blocks[0].size,
                        file.blocks[0].hash.clone()
                    ).unwrap_or_else(|e| {
                        panic!("Error sending block request: {}", e);
                    });

                    let block_state = FileFetchState {
                        file: fs_file,
                        size: file.size as u64,
                        read_bytes: 0,
                        all_blocks: file.blocks.clone(),
                        current_outstanding_idx: 0,
                        folder_id: index.folder.clone(),
                        path: file.name.clone(),
                    };

                    if let Some(state) = fetch_state {
                        state.request_map.insert(req_id, block_state);
                    } else {
                        let mut m = HashMap::new();
                        m.insert(req_id, block_state);
                        *fetch_state = Some(BlockFetchState { request_map: m });
                    }
                }
            }
        }


        eprintln!("index entries: {} / {}",
                index.files.last().unwrap().sequence,
                folder_info.max_remote_seq);

        if index.files[index.files.len() - 1].sequence >= folder_info.max_remote_seq {
            // Note that this assumes nothing changed in between when we got the
            // cluster config and now.
            // It also assumes that the files in each message are sorted by
            // sequence number.
            debug!("got last index update for this folder");
            debug!("index_n = {}; folders_by_id = {}",
                   index_recv_state.index_n,
                   self.folders_by_id.len());

            index_recv_state.index_n += 1;
        }
    }

    fn handle_response(
        &self,
        response: &mut proto::Response,
        session: &mut stget::session::Session,
        mut fetch_state: FileFetchState,
    ) -> Option<(i32, FileFetchState)> {

        fetch_state.read_bytes += response.data.len() as u64;
        eprintln!("{:?}: received block {} / {} -- {} / {} bytes",
                  fetch_state.path,
                  response.id + 1,
                  fetch_state.all_blocks.len(),
                  fetch_state.read_bytes,
                  fetch_state.size);

        match response.code.unwrap() {
            proto::ErrorCode::NO_ERROR => (),
            proto::ErrorCode::GENERIC => {
                eprintln!("Error: remote host says there is some unspecified error");
                return None;
            }
            proto::ErrorCode::NO_SUCH_FILE => {
                eprintln!("Error: remote host says there is no such file");
                return None;
            }
            proto::ErrorCode::INVALID_FILE => {
                eprintln!("Error: remote host says invalid file");
                return None;
            }
        }

        std::io::Write::write_all(&mut fetch_state.file, &response.data)
            .expect("file write error");

        if fetch_state.current_outstanding_idx == fetch_state.all_blocks.len() - 1 {
            assert_eq!(fetch_state.size, fetch_state.read_bytes);
            eprintln!("fetched {} bytes", fetch_state.size);
            return None;
        }

        let idx = fetch_state.current_outstanding_idx + 1;
        fetch_state.current_outstanding_idx += 1;
        let req_id = session.write_block_request(
            fetch_state.folder_id.clone(),
            fetch_state.path.clone(),
            fetch_state.all_blocks[idx].offset,
            fetch_state.all_blocks[idx].size,
            fetch_state.all_blocks[idx].hash.clone()
        ).unwrap_or_else(|e| {
            panic!("Error sending block request: {}", e);
        });

        Some((req_id, fetch_state))
    }
}

#[allow(dead_code)]
fn hexdump(data: &[u8]) {
    for i in 0 ..= (data.len() / 16) {
        eprint!("{:04x}  ", i * 16);
        for h in 0 .. 16 {
            if i * 16 + h < data.len() {
                eprint!("{:02x} ", data[i * 16 + h]);
            } else {
                eprint!("   ");
            }
            if h == 8 {
                eprint!(" ");
            }
        }
        eprint!("  |");
        for h in 0 .. 16 {
            if i * 16 + h < data.len() {
                let c = match data[i * 16 + h] {
                    0..=0x19 | 0x7f..=0xff => '.',
                    other => other as char,
                };
                eprint!("{}", c);
            } else {
                eprint!(" ");
            }
        }
        eprintln!("|");
    }
}
