use sodiumoxide::crypto::secretbox::NONCEBYTES;
use sodiumoxide::crypto::hash::sha256::Digest;

pub fn set_variables() -> (Vec<u8>, [u8; NONCEBYTES], Digest) {
    let pass: Digest = Digest([
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
        27, 28, 29, 30, 31, 32,
    ]);

    let nonce: [u8; NONCEBYTES] = [
        0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
        0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
    ];

    let buffer: Vec<u8> = vec![];

    return (buffer, nonce, pass);
}
