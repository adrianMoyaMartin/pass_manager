pub fn list_command(buffer: &[u8]) {
    for x in buffer.split(|x| *x == 10).filter(|x| !x.is_empty()).map(|x| &x[0..x.iter().position(|x| *x==0).expect("storage corrupt")]) {
        println!("{}", String::from_utf8_lossy(x));
    }
}
