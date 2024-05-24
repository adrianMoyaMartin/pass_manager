#[allow(unused_must_use)]
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use sodiumoxide::crypto::hash::sha256::Digest;
use sodiumoxide::crypto::hash;

use std::fs::{ self, File, OpenOptions };
use std::io::{ Read, Seek, Write };
use std::path::Path;

mod utilities;
use crate::utilities::file_handler::*;
use crate::utilities::variables::set_variables;

mod commands;
use crate::commands::add_command::add_command;
use crate::commands::find_command::find_command;
use crate::commands::list_command::list_command;

const PATH_MASTER: &str = "master.txt";

const HELP: &str =
    "list of commands:
    >> help: recites all commands,
    >> add [name under which it will be stored] [password to be saved]: will encrypt and save a password and store it under a given name,
    >> find [name given to password before it was saved]: will fetch and unencrypt a password using the name given to it,
    >> list: will enumerate all the names assigned to passwords,";

fn main() {
    let time = std::time::SystemTime::now();
        
    let (nonce, mut pass) = set_variables();
    
    let mut rl = DefaultEditor::new().expect("couldn't create reader");

    let mut file = make_file().expect("couldn't create/open file");
    
    // let mut reader = BufReader::new(&file);
    // let mut writer = BufWriter::new(file);

    if Path::new(PATH_MASTER).exists() == false {
        let mut master = OpenOptions::new().read(true).write(true).create(true).open(PATH_MASTER).expect("couldnt read master file");

        let readline = rl.readline("New User! Please chose a master password: ");
        
        match readline {
            Ok(line) => {
                let pswd = line.as_bytes();
                let pswd_hash: Digest = hash::sha256::hash(pswd);
                
                pass = pswd_hash;
                
                master.write(pswd_hash.as_ref()).expect("couldn't write to master");
                println!("password set");
            }
            Err(err) => {
                eprintln!("Error whilst setting password please restart program: {}", err);
                return;
            }
        }
        
        let metadata = master.metadata().expect("couldn't get metadata");
        let mut perms = metadata.permissions();
        
        perms.set_readonly(true);
        
        fs::set_permissions(PATH_MASTER, perms).expect("couldn't set perms");
    } else {
        let mut binding_master = File::options();
        
        let mut master = binding_master.read(true).open(PATH_MASTER).expect("couldn't read master");
        
        let readline = rl.readline("Your master password: ");
        
        match readline {
            Ok(line) => {
                let mut buf: Vec<u8> = vec![];
                let pswd = line.as_bytes();
                let pswd_hash = hash::sha256::hash(pswd);

                master.read_to_end(&mut buf).expect("couldn't read master file");
                if pswd_hash.as_ref() == buf {
                    pass = pswd_hash;
                    println!("Correct password");
                } else {
                    println!("Incorrect password");
                    return;
                }
            }
            Err(err) => {
                println!("Error whilst inputting password please restart program: {}", err);
            }
        }
    }

    let mut buffer = vec![];
    file.read_to_end(&mut buffer).expect("couldn't read");

    let time_taken = time.elapsed().expect("error during time conversion");
    println!("seconds taken: {}, ms taken: {}", time_taken.as_secs_f32(), time_taken.as_millis());

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line.len() == 0 {
                    println!("Must not be empty");
                } else {
                    let fractured_args: Vec<&str> = line.split_ascii_whitespace().collect();
                    match fractured_args[0] {
                        "add" => {
                            add_command(fractured_args, &mut file, nonce, pass).expect("couldn't add to file");
                            file.flush().expect("couldn't flush");
                        }
                        "find" => {
                            find_command(fractured_args, buffer.clone(), pass, nonce).expect("couldn't find");
                        }
                        "list" => {
                            let mut buf = vec![];
                            file.seek(std::io::SeekFrom::Start(0)).expect("couldn't seek to beginning");
                            file.read_to_end(&mut buf).expect("couldn't read file");
                            list_command(&buf);
                        }
                        "help" => {
                            println!("{}", HELP);
                        }
                        _ => {
                            println!("invalid command");
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL+C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL+D");
                break;
            }
            Err(err) => {
                println!("ERR: {}", err);
            }
        }
    }
}