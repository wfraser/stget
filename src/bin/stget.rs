#![allow(unknown_lints)]

extern crate byteorder;
extern crate clap;
extern crate env_logger;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;
extern crate protobuf;
extern crate stget;

use std::collections::HashMap;
use byteorder::{ByteOrder, NetworkEndian};
use stget::syncthing_proto as proto;

error_chain! {
    foreign_links {
        Io(std::io::Error);
    }

    errors {
    }
}

fn main() {
    env_logger::init();

    let args = clap::App::new(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about("experimental Syncthing file retrieval program")
            .arg(clap::Arg::with_name("address")
                    .help("Address of the remote host. Port 22000 is used if unspecified.")
                    .required(true)
                    .index(1))
            .arg(clap::Arg::with_name("device_id")
                    .help("Device ID of the remote host.")
                    .required(true)
                    .index(2))
            .arg(clap::Arg::with_name("path")
                    .help("File path to fetch.")
                    .index(3))
            .arg(clap::Arg::with_name("list")
                    .short("l")
                    .long("list")
                    .takes_value(false)
                    .help("List all files on the remote end."))
            .group(clap::ArgGroup::with_name("path_or_list")
                    .args(&["path", "list"])
                    .required(true))
            .get_matches();

    let host_and_port = match args.value_of("address").unwrap() {
        host if host.contains(':') => host.to_owned(),
        host => {
            debug!("no port specified; assuming 22000");
            format!("{}:22000", host)
        }
    };

    let device_id = args.value_of("device_id").unwrap();
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
        remote_host_and_port: host_and_port.to_string(),
        remote_device_id: device_id.to_string(),
        local_device_name: None,
        client_cert: cert,
        private_key: key,
    }.connect().expect("Failed to create TLS session");

    session.write_hello().unwrap();

    let mut program_state = ProgramState {
        remote_cert_hash,
        folders_by_id: HashMap::new(),
        mode: if args.is_present("list") {
            Mode::List
        } else {
            Mode::Fetch(args.value_of("path").unwrap().to_owned())
        },
        protocol_state: Some(State::ExpectHello),
    };

    let mut data = vec![];
    while let Ok((r, w)) = session.complete_io() {
        if r == 0 && w == 0 {
            break;
        }
        debug!("r = {}, w = {}", r, w);
        let data_len = data.len();
        match session.read_to_end(&mut data) {
            Err(stget::Error(stget::ErrorKind::Io(ref e), _))
                    if e.kind() == std::io::ErrorKind::ConnectionAborted => {
                eprintln!("connection closed");
                break;
            },
            Err(e) => {
                debug!("len was {}, now is {}", data_len, data.len());
                panic!("read error: {:?}", e)
            },
            Ok(n) => {
                debug!("read {}", n);
                /*
                if w == 0 && n == r {
                    eprintln!("done");
                    break;
                }
                */

                if n > 0 {
                    process_network_data(&mut program_state, &mut session, &mut data);
                    if program_state.protocol_state.is_none() {
                        break;
                    }
                }
            }
        }
    }

    if program_state.protocol_state == Some(State::ExpectHello)
            || program_state.protocol_state == Some(State::ExpectClusterConfig)
    {
        eprintln!("Remote declined to talk with us.");
        if program_state.protocol_state == Some(State::ExpectHello) {
            let (_len, remote_hello): (usize, proto::Hello) =
            stget::session::Session::read_hello(&data).unwrap_or_else(|e| {
                eprintln!("error reading remote hello: {}", e);
                panic!(e);
            });
            eprintln!("Remote is \"{}\", running {} {}",
                    remote_hello.device_name,
                    remote_hello.client_name,
                    remote_hello.client_version);
        }
    }
}

fn process_network_data(program: &mut ProgramState, session: &mut stget::session::Session, data: &mut Vec<u8>) {
    match program.protocol_state.take() {
        Some(State::ExpectHello) => {
            debug!("got hello");
            program.handle_hello(data);
        },
        Some(State::ExpectClusterConfig) => {
            debug!("got remote cluster config");
            program.handle_cluster_config(data, session);
        },
        Some(State::ExpectIndex(index_n, fetch_state)) => {
            debug!("got index {}", index_n);
            program.handle_index(data, index_n, fetch_state, session);
        }
        Some(State::FetchBlocks(fetch_state)) => {
            debug!("got block response");
            program.handle_response(data, fetch_state, session);
        }
        None => panic!("bad state"),
    }
}

#[derive(Debug)]
struct ProgramState {
    remote_cert_hash: Vec<u8>,
    folders_by_id: HashMap<String, FolderInfo>,
    mode: Mode,
    protocol_state: Option<State>,
}

#[derive(Debug)]
enum Mode {
    List,
    Fetch(String),
}

#[derive(Debug, Clone, PartialEq)]
enum State {
    ExpectHello,
    ExpectClusterConfig,
    ExpectIndex(usize, Option<BlockFetchState>),
    FetchBlocks(BlockFetchState),
}

#[derive(Debug)]
struct FolderInfo {
    label: String,
    max_remote_seq: i64,
}

#[derive(Debug, Clone, PartialEq)]
struct BlockFetchState {
    file_data: Vec<u8>,
    file_size: u64,
    folder_id: String,
    path: String,
    block_info: Vec<proto::BlockInfo>,
    current_outstanding: usize,
}

impl ProgramState {
    pub fn handle_hello(&mut self, data: &mut Vec<u8>) {
        let (len, remote_hello): (usize, proto::Hello) =
            stget::session::Session::read_hello(data).unwrap_or_else(|e| {
                eprintln!("error reading remote hello: {}", e);
                panic!(e);
            });
        eprintln!("Remote is \"{}\", running {} {}",
                remote_hello.device_name,
                remote_hello.client_name,
                remote_hello.client_version);
        data.drain(0..len);

        // Wait to send cluster config until we read the remote one.
        self.protocol_state = Some(State::ExpectClusterConfig);
    }

    pub fn handle_cluster_config(
        &mut self, data: &mut Vec<u8>,
        session: &mut stget::session::Session,
    ) {
        let (len, msgtype, message) = stget::session::Session::read_message(data)
            .unwrap_or_else(|e| {
                eprintln!("Error reading remote cluster config: {}", e);
                panic!(e);
            });
        data.drain(0..len);

        let remote_cluster_config: &proto::ClusterConfig = match msgtype {
            proto::MessageType::CLUSTER_CONFIG => message.as_any().downcast_ref().unwrap(),
            other => {
                eprintln!("unexpected message type {:?}; wanted CLUSTER_CONFIG", other);
                return;
            }
        };

        debug!("remote cluster config: {:#?}", remote_cluster_config);

        let mut cluster_config = proto::ClusterConfig::new();

        for folder in remote_cluster_config.get_folders() {
            for device in folder.get_devices() {
                let device_cert_hash: &[u8] = device.get_id();
                if device_cert_hash == self.remote_cert_hash.as_slice() {
                    self.folders_by_id.insert(
                        folder.get_id().to_owned(),
                        FolderInfo {
                            label: folder.get_label().to_owned(),
                            max_remote_seq: device.get_max_sequence(),
                        });
                }
            }
        }

        match self.mode {
            Mode::List => {
                // make a cluster config with all the folders in the remote
                for remote_folder in remote_cluster_config.get_folders() {
                    let mut folder = proto::Folder::new();
                    folder.set_id(remote_folder.get_id().to_owned());
                    folder.set_label(remote_folder.get_label().to_owned());
                    folder.set_read_only(true);
                    folder.set_ignore_permissions(true);
                    folder.set_ignore_delete(true);
                    folder.set_disable_temp_indexes(true);
                    cluster_config.mut_folders().push(folder);
                }
            },
            Mode::Fetch(ref path) => {
                let folder_name = path.splitn(2, '/').next().unwrap();

                let mut folder_id = None;
                for folder in remote_cluster_config.get_folders() {
                    if folder.get_label() == folder_name {
                        folder_id = Some(folder.get_id());
                        break;
                    }
                }
                if folder_id.is_none() {
                    eprintln!("The remote computer is not offering a folder with the specified name (\"{}\").", folder_name);
                    eprintln!("it offered:");
                    for folder in remote_cluster_config.get_folders() {
                        eprintln!("    {} ({})", folder.get_label(), folder.get_id());
                    }
                    std::process::exit(1);
                }

                let mut folder = proto::Folder::new();
                folder.set_id(folder_id.unwrap().to_owned());
                folder.set_label(folder_name.to_owned());
                folder.set_read_only(true);
                folder.set_ignore_permissions(true);
                folder.set_ignore_delete(true);
                folder.set_disable_temp_indexes(true);
                cluster_config.mut_folders().push(folder);
            }
        }

        debug!("sending cluster config");
        session.write_message(
                &cluster_config,
                proto::MessageType::CLUSTER_CONFIG)
            .unwrap_or_else(|e| {
                eprintln!("error sending our cluster config: {}", e);
                panic!(e);
            });

        eprintln!("receiving folder index");
        self.protocol_state = Some(State::ExpectIndex(0, None));
    }

    pub fn handle_index(
        &mut self,
        data: &mut Vec<u8>,
        index_n: usize,
        mut fetch_state: Option<BlockFetchState>,
        session: &mut stget::session::Session,
    ) {
        let header_len = NetworkEndian::read_u16(&data[0..2]) as usize;
        let body_len = NetworkEndian::read_u32(&data[
            2 + header_len as usize
                .. 2 + header_len as usize + 4]) as usize;
        let data_len = 2 + header_len + 4 + body_len;
        if data.len() < data_len {
            debug!("not enough data; reading more (need {}, have {})",
                    data_len, data.len());
            self.protocol_state = Some(State::ExpectIndex(index_n, fetch_state));
            return;
        }

        let (input_pos, msgtype, message) = stget::session::Session::read_message(data)
            .unwrap_or_else(|e| {
                eprintln!("Error reading remote message: {}", e);
                panic!(e);
            });
        debug!("{} bytes read", input_pos);
        assert_eq!(data_len, input_pos);

        let index: &proto::Index = match msgtype {
            proto::MessageType::INDEX => message.as_any().downcast_ref().unwrap(),
            proto::MessageType::INDEX_UPDATE => unsafe {
                // Horrible hack relying on the fact that Index and IndexUpdate are the same
                &*(message.as_any().downcast_ref::<proto::IndexUpdate>().unwrap()
                    as *const proto::IndexUpdate
                    as *const proto::Index)
            },
            proto::MessageType::PING => {
                debug!("got a ping message");
                self.protocol_state = Some(State::ExpectIndex(index_n, fetch_state));
                return;
            },
            proto::MessageType::CLOSE => {
                let close: &proto::Close = message.as_any().downcast_ref().unwrap();
                debug!("got a close message: {:?}", close);
                return;
            },
            proto::MessageType::RESPONSE if fetch_state.is_some() => {
                debug!("in handle_index, got a RESPONSE message");
                // ignore the protocol state and just handle it
                self.handle_response(data, fetch_state.unwrap(), session);
                return;
            },
            other => {
                eprintln!("got an unexpected message type: {:?}", other);
                return;
            }
        };

        data.drain(0..data_len);

        debug!("remote index: {:#?}", index);

        let folder_info = &self.folders_by_id[index.get_folder()];
        let files = index.get_files();

        for file in files {
            if file.get_field_type() == proto::FileInfoType::DIRECTORY
                    || file.get_deleted() {
                continue;
            }

            let path = format!("{}/{}", folder_info.label, file.get_name());
            match self.mode {
                Mode::List => {
                    println!("{}", path);
                }
                Mode::Fetch(ref check_path) if check_path == &path => {
                    debug!("found the file");
                    eprintln!("requesting the file");

                    // request the first block and do a state transition

                    let state = BlockFetchState {
                        // 32-bit targets gonna have a bad time here
                        file_data: Vec::with_capacity(file.size as usize),
                        file_size: file.size as u64,
                        folder_id: index.folder.clone(),
                        path: file.get_name().to_owned(),
                        block_info: file.get_Blocks().to_owned(),
                        current_outstanding: 0,
                    };

                    session.write_block_request(
                        state.folder_id.clone(),
                        state.path.clone(),
                        state.block_info[0].offset,
                        state.block_info[0].size,
                        state.block_info[0].hash.clone(),
                    ).unwrap_or_else(|e| {
                        eprintln!("Error sending block request: {}", e);
                        panic!(e);
                    });

                    fetch_state = Some(state);
                }
                _ => ()
            }
        }


        eprintln!("index entries: {} / {}",
                files.last().unwrap().get_sequence(),
                folder_info.max_remote_seq);

        if files[files.len() - 1].get_sequence() >= folder_info.max_remote_seq {
            // Note that this assumes nothing changed in between when we got the
            // cluster config and now.
            // It also assumes that the files in each message are sorted by
            // sequence number.
            debug!("got last index update for this folder");
            debug!("index_n = {}; folders_by_id = {}", index_n, self.folders_by_id.len());

            if let Mode::Fetch(_) = self.mode {
                if let Some(fetch_state) = fetch_state {
                    self.protocol_state = Some(State::FetchBlocks(fetch_state));
                } else {
                    eprintln!("No matching file was found in the directory index.");
                    self.protocol_state = None;
                }
            } else if index_n + 1 == self.folders_by_id.len() {
                // all done :)
                self.protocol_state = None;
            } else {
                self.protocol_state = Some(State::ExpectIndex(index_n + 1, fetch_state));
            }
        } else {
            self.protocol_state = Some(State::ExpectIndex(index_n, fetch_state));
        }
    }

    pub fn handle_response(
        &mut self,
        data: &mut Vec<u8>,
        mut fetch_state: BlockFetchState,
        session: &mut stget::session::Session,
    ) {
        let header_len = NetworkEndian::read_u16(&data[0..2]) as usize;
        let body_len = NetworkEndian::read_u32(&data[
            2 + header_len as usize
                .. 2 + header_len as usize + 4]) as usize;
        if data.len() < header_len + body_len {
            debug!("not enough data; reading more (need {}, have {})",
                    header_len + body_len, data.len());
            self.protocol_state = Some(State::FetchBlocks(fetch_state));
            return;
        }

        let (input_pos, msgtype, mut message) = match stget::session::Session::read_message(data) {
            Ok(stuff) => stuff,
            Err(stget::Error(
                stget::ErrorKind::ProtoBuf(
                    protobuf::ProtobufError::WireError(
                        protobuf::error::WireError::UnexpectedEof)),
                _)) => {
                debug!("we have {:#x} bytes, but need to read more", data.len());
                self.protocol_state = Some(State::FetchBlocks(fetch_state));
                return;
            }
            Err(e) => {
                eprintln!("Error reading block response: {}", e);
                panic!(e);
            }
        };

        let response: &mut proto::Response = match msgtype {
            proto::MessageType::RESPONSE => message.as_any_mut().downcast_mut().unwrap(),
            proto::MessageType::INDEX_UPDATE => {
                debug!("in handle_response, got an INDEX_UPDATE");
                // ignore the protocol state and just handle it
                self.handle_index(data, 0, Some(fetch_state), session);
                return;
            }
            other => {
                eprintln!("unexpected message type {:?}; wanted RESPONSE", other);
                return;
            }
        };

        data.drain(0..input_pos);

        eprintln!("received block {} / {} -- {} / {} bytes",
                  response.id + 1,
                  fetch_state.block_info.len(),
                  fetch_state.file_data.len() + response.data.len(),
                  fetch_state.file_size);
        match response.code {
            proto::ErrorCode::NO_ERROR => (),
            proto::ErrorCode::GENERIC => {
                eprintln!("Error: remote host says there is some unspecified error");
                self.protocol_state = None;
                return;
            }
            proto::ErrorCode::NO_SUCH_FILE => {
                eprintln!("Error: remote host says there is no such file");
                self.protocol_state = None;
                return;
            }
            proto::ErrorCode::INVALID_FILE => {
                eprintln!("Error: remote host says invalid file");
                self.protocol_state = None;
                return;
            }
        }

        fetch_state.file_data.append(&mut response.take_data());

        if fetch_state.current_outstanding == fetch_state.block_info.len() - 1 {
            assert_eq!(fetch_state.file_size as usize, fetch_state.file_data.len());
            eprintln!("fetched {} bytes", fetch_state.file_size);
            std::io::Write::write_all(&mut std::io::stdout(), &fetch_state.file_data)
                .expect("write error");
            self.protocol_state = None;
            return;
        }

        let idx = fetch_state.current_outstanding + 1;
        fetch_state.current_outstanding += 1;
        session.write_block_request(
            fetch_state.folder_id.clone(),
            fetch_state.path.clone(),
            fetch_state.block_info[idx].offset,
            fetch_state.block_info[idx].size,
            fetch_state.block_info[idx].hash.clone()
        ).unwrap_or_else(|e| {
            eprintln!("Error sending block request: {}", e);
            panic!(e);
        });

        self.protocol_state = Some(State::FetchBlocks(fetch_state));
    }
}

#[allow(dead_code)]
fn hexdump(data: &[u8]) {
    for i in 0 .. ((data.len() / 16) + 1) {
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
                    0...0x20 | 0x7f..=0xff => '.',
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
