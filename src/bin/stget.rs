#![allow(unknown_lints)]

extern crate byteorder;
extern crate clap;
extern crate env_logger;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;
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
    env_logger::init().unwrap();

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
        remote_cert_hash: remote_cert_hash,
        folders_by_id: HashMap::new(),
        list_mode: args.is_present("list"),
        path: args.value_of("path").map(|s| s.to_owned()),
        protocol_state: Some(State::ExpectHello),
        file_info: None,
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
            hexdump(data);
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
            program.protocol_state = Some(State::ExpectClusterConfig);
        },
        Some(State::ExpectClusterConfig) => {
            hexdump(data);
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
                    if device_cert_hash == program.remote_cert_hash.as_slice() {
                        program.folders_by_id.insert(
                            folder.get_id().to_owned(),
                            FolderInfo {
                                label: folder.get_label().to_owned(),
                                max_remote_seq: device.get_max_sequence(),
                            });
                    }
                }
            }

            if program.list_mode {
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
            } else {
                let folder_name = program.path.as_ref().unwrap()
                    .splitn(2, '/').next().unwrap();

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

            debug!("sending cluster config");
            session.write_message(
                    cluster_config,
                    proto::MessageType::CLUSTER_CONFIG)
                .unwrap_or_else(|e| {
                    eprintln!("error sending our cluster config: {}", e);
                    panic!(e);
                });

            program.protocol_state = Some(State::ExpectIndex(0));
        },
        Some(State::ExpectIndex(index_n)) => {
            let header_len = NetworkEndian::read_u16(&data[0..2]) as usize;
            let body_len = NetworkEndian::read_u32(&data[
                2 + header_len as usize
                    .. 2 + header_len as usize + 4]) as usize;
            if data.len() < header_len + body_len {
                debug!("not enough data; reading more (need {}, have {})",
                        header_len + body_len, data.len());
                program.protocol_state = Some(State::ExpectIndex(index_n));
                return;
            }
            //hexdump(data);

            let (len, msgtype, message) = stget::session::Session::read_message(data)
                .unwrap_or_else(|e| {
                    eprintln!("Error reading remote message: {}", e);
                    panic!(e);
                });
            debug!("{} bytes read", len);
            data.drain(0..len);

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
                    program.protocol_state = Some(State::ExpectIndex(index_n));
                    return;
                },
                proto::MessageType::CLOSE => {
                    let close: &proto::Close = message.as_any().downcast_ref().unwrap();
                    debug!("got a close message: {:?}", close);
                    return;
                },
                other => {
                    eprintln!("got an unexpected message type: {:?}", other);
                    return;
                }
            };

            debug!("remote index: {:#?}", index);

            let folder_info = &program.folders_by_id[index.get_folder()];
            let files = index.get_files();

            for file in files {
                if file.get_field_type() == proto::FileInfoType::DIRECTORY
                        || file.get_deleted() {
                    continue;
                }

                let path = format!("{}/{}", folder_info.label, file.get_name());
                if program.list_mode {
                    println!("{}", path);
                } else if &path == program.path.as_ref().unwrap() {
                    debug!("found the file");
                    assert_eq!(None, program.file_info);
                    program.file_info = Some(file.clone());
                }
            }

            if files[files.len() - 1].get_sequence() >= folder_info.max_remote_seq {
                // Note that this assumes nothing changed in between when we got the
                // cluster config and now.
                // It also assumes that the files in each message are sorted by
                // sequence number.
                debug!("got last index update for this folder");
                debug!("index_n = {}; folders_by_id = {}", index_n, program.folders_by_id.len());
                if program.path.is_some() || index_n + 1 == program.folders_by_id.len() {
                    debug!("at last folder; ending");
                    eprintln!("File info: {:#?}", program.file_info);

                    // TODO: write a request for the first block (maybe all the blocks?)
                    // and fill in the vector for the ExpectBlocks state

                    // let request_id = session.write_block_request(folder, path, offset, size,
                    //      hash)?;

                    program.protocol_state = Some(State::ExpectBlocks(vec![]));
                    unimplemented!("requesting blocks");
                } else {
                    program.protocol_state = Some(State::ExpectIndex(index_n + 1));
                }
            } else {
                program.protocol_state = Some(State::ExpectIndex(index_n));
            }
        }
        Some(State::ExpectBlocks(_blocks)) => {
            unimplemented!("requesting blocks");
        }
        None => panic!("bad state"),
    }
}

#[derive(Debug)]
struct ProgramState {
    remote_cert_hash: Vec<u8>,
    folders_by_id: HashMap<String, FolderInfo>,
    list_mode: bool,
    path: Option<String>,
    file_info: Option<proto::FileInfo>,
    protocol_state: Option<State>,
}

#[derive(Debug, Clone, PartialEq)]
enum State {
    ExpectHello,
    ExpectClusterConfig,
    ExpectIndex(usize),
    ExpectBlocks(Vec<Block>), // maybe a map keyed on request_id would be better?
}

#[derive(Debug, Clone, PartialEq)]
enum Block {
    Outstanding(i32), // request ID
    Data(Vec<u8>),
}

#[derive(Debug)]
struct FolderInfo {
    label: String,
    max_remote_seq: i64,
}

fn hexdump(data: &[u8]) {
    for i in 0 .. ((data.len() / 16) + 1) {
        print!("{:04x}  ", i * 16);
        for h in 0 .. 16 {
            if i * 16 + h < data.len() {
                print!("{:02x} ", data[i * 16 + h]);
            } else {
                print!("   ");
            }
            if h == 8 {
                print!(" ");
            }
        }
        print!("  |");
        for h in 0 .. 16 {
            if i * 16 + h < data.len() {
                let mut c = data[i * 16 + h] as char;
                if c < ' ' {
                    c = '.'
                }
                print!("{}", c);
            } else {
                print!(" ");
            }
        }
        println!("|");
    }
}
