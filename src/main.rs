#[allow(unused_must_use)]
use rustyline::error::ReadlineError;
use rustyline::{ DefaultEditor, Result };

use sodiumoxide::crypto::hash::sha256::Digest;
use sodiumoxide::crypto::hash;

use std::fs::{ self, File };
use std::io::{ BufReader, BufWriter };
use std::io::{ Read, Write };
use std::path::Path;

mod utilities {
    pub mod file_handler;
    pub mod variables;
}
use crate::utilities::file_handler::*;
use crate::utilities::variables::set_variables;

mod commands {
    pub mod add_command;
    pub mod find_command;
    pub mod list_command;
}
use crate::commands::add_command::add_command;
use crate::commands::find_command::find_command;
use crate::commands::list_command::list_command;

const PATH: &str = "src/storage.txt";
const PATH_MASTER: &str = "src/master.txt";

const HELP: &str =
    "list of commands:
    >> help: recites all commands,
    >> add [name under which it will be stored] [password to be saved]: will encrypt and save a password and store it under a given name,
    >> find [name given to password before it was saved]: will fetch and unencrypt a password using the name given to it,
    >> list: will enumerate all the names assigned to passwords,";

fn main() -> Result<()> {
    let time = std::time::SystemTime::now();

    let (mut buffer, nonce, mut pass) = set_variables();

    let mut rl = DefaultEditor::new()?;

    let binding_file: fs::OpenOptions = File::options();

    let mut file = make_file(binding_file.clone());

    let mut reader = BufReader::new(file.try_clone()?);
    let mut writer = BufWriter::new(file.try_clone()?);

    if Path::new(PATH_MASTER).exists() == false {
        let mut binding_master = File::options();

        let mut master = binding_master.read(true).write(true).create(true).open(PATH_MASTER)?;

        let readline = rl.readline("New User! Please chose a master password: ");

        match readline {
            Ok(line) => {
                let pswd = line.as_bytes();
                let pswd_hash: Digest = hash::sha256::hash(pswd);

                pass = pswd_hash;

                master.write(pswd_hash.as_ref())?;
                println!("password set");
            }
            Err(err) => {
                println!("Error whilst setting password please restart program: {}", err);
                return Ok(());
            }
        }

        let metadata = master.metadata()?;
        let mut perms = metadata.permissions();

        perms.set_readonly(true);

        fs::set_permissions(PATH_MASTER, perms)?;
    } else {
        let mut binding_master = File::options();

        let mut master = binding_master.read(true).open(PATH_MASTER)?;

        let readline = rl.readline("Your master password: ");

        match readline {
            Ok(line) => {
                let mut buf: Vec<u8> = vec![];
                let pswd = line.as_bytes();
                let pswd_hash = hash::sha256::hash(pswd);

                master.read_to_end(&mut buf)?;
                if pswd_hash.as_ref() == buf {
                    pass = pswd_hash;
                    println!("Correct password");
                } else {
                    println!("Incorrect password");
                    return Ok(());
                }
            }
            Err(err) => {
                println!("Error whilst inputting password please restart program: {}", err);
            }
        }
    }

    reader.read_to_end(&mut buffer)?;

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
                            add_command(fractured_args, &mut writer, nonce, pass)?;

                            (file, writer) = update_file(&mut buffer, binding_file.clone());
                        }
                        "find" => {
                            find_command(fractured_args, buffer.clone(), pass, nonce)?;
                        }
                        "list" => {
                            list_command(buffer.clone());
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

    let metadata = file.metadata().expect("failed to read metadata");
    let mut perms: fs::Permissions = metadata.permissions();

    perms.set_readonly(true);

    fs::set_permissions(PATH, perms)?;

    Ok(())
}
