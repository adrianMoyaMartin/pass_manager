use sodiumoxide::crypto::secretbox::Nonce;
use sodiumoxide::crypto::secretbox;

use std::ops::Deref;
use sodiumoxide::crypto::hash::sha256::Digest;

use rustyline::Result;

pub fn find_command(fractured_args: Vec<&str>, buffer: Vec<u8>, pass: Digest, nonce: [u8; 24]) -> Result<()> {
    if fractured_args.len() == 2 {
        let contents = &buffer;

        let mut list: Vec<Vec<u8>> = vec![];

        let mut is_found = false;
        let mut found_index: usize = 0;

        for (_, x) in contents.clone().into_iter().enumerate() {
            if x == 10 || x == 0 || list.is_empty() {
                if !list.is_empty() {
                    let text = String::from_utf8_lossy(&list.last().unwrap());

                    if text.deref() == fractured_args[1] {
                        is_found = true;
                        found_index = list.len();
                    }
                }
                list.push(vec![]);
                if list.is_empty() && x != 10 && x != 0 {
                    list.last_mut().unwrap().push(x);
                }
            } else {
                list.last_mut().unwrap().push(x);
            }
        }
        if is_found == true {
            let unsealed_pass = secretbox
                ::open(
                    &list[found_index],
                    &Nonce::from_slice(&nonce).unwrap(),
                    &secretbox::Key(pass.0)
                )
                .expect("error while decrypting: ");
            println!("{:?}", String::from_utf8(unsealed_pass)?);
        } else {
            println!("query not found");
        }
    } else {
        println!("PLEASE FILL IN ALL FIELDS");
    }

    Ok(())
}
