use sodiumoxide::crypto::secretbox::Nonce;
use sodiumoxide::crypto::secretbox;

use std::io::Write;
use std::io::Result;
use sodiumoxide::crypto::hash::sha256::Digest;


pub fn add_command<T: Write>(fractured_args: Vec<&str>, file: &mut T, nonce: [u8; 24], pass: Digest) -> Result<()> {
    if fractured_args.len() == 3 {
        let safe_pass = secretbox::seal(
            fractured_args[2].as_ref(),
            &Nonce::from_slice(&nonce).unwrap(),
            &secretbox::Key(pass.0)
        );

        let to_write = fractured_args[1];

        file.write_all(&to_write.as_bytes())?;
        file.write_all(&[0])?;
        file.write_all(&safe_pass)?;
        file.write_all("\n".as_bytes())?;

        file.flush()?;
        println!("Password saved under: {}", fractured_args[1]);
    } else {
        println!("All arguments must not be empty");
    }
    Ok(())
}
