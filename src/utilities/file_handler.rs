use std::{
    fs::{ self, File, OpenOptions },
    io::{ self, BufReader, BufWriter, Read },
    path::Path,
    process::Command,
};

const PATH: &str = "src/storage.txt";

pub fn make_file(mut binding_file: OpenOptions) -> File {
    let file: File = match Path::new(PATH).exists() {
        true => {
            let x = binding_file
                .read(true)
                .create(false)
                .write(false)
                .open(PATH)
                .expect("failed to open file");

            if cfg!(windows) {
                match run_as_admin() {
                    Ok(_) => {
                        println!("Program executed with elevated privileges.");
                    }
                    Err(err) => {
                        println!("Program failed to be executed with elevated privileges: {}", err);
                    }
                }
            }
            let metadata = x.metadata().expect("failed to read metadata");
            let mut perms: fs::Permissions = metadata.permissions();
            perms.set_readonly(false);
            fs::set_permissions(PATH, perms).expect("failed to set permissions");

            binding_file
                .read(true)
                .create(false)
                .write(true)
                .open(PATH)
                .expect("failed to open file")
        }
        false => {
            binding_file
                .read(true)
                .create(true)
                .write(true)
                .open(PATH)
                .expect("failed to open file")
        }
    };
    return file;
}

pub fn update_file(
    mut buffer: &mut Vec<u8>,
    mut binding_file: OpenOptions
) -> (File, BufWriter<File>) {
    let file = binding_file
        .read(true)
        .create(true)
        .write(true)
        .open(PATH)
        .expect("failed to create file");

    let mut reader = BufReader::new(file.try_clone().expect("failed to create BufReader"));
    let writer = BufWriter::new(file.try_clone().expect("failed to create BufWriter"));

    reader.read_to_end(&mut buffer).expect("failed whilst reading file");

    (file, writer)
}

fn run_as_admin() -> io::Result<()> {
    let mut command = Command::new("cmd");
    command
        .arg("/C")
        .arg("start")
        .arg("/wait")
        .arg("runas")
        .arg("/user:Administrator")
        .arg("\"cmd /c\"")
        .arg("\"pass_manager.exe\"");

    command.status()?;
    Ok(())
}
