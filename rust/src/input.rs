use std::io;
use std::io::Read;

/// return of zero means there was no input available
pub fn read_byte() -> u8 {
    let mut x: [u8;1] = [0];
    match io::stdin().read(&mut x) {
        Ok(_) => {},
        Err(y) => {},
    };
    return x[0];
}