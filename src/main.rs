use std::io::{self, Read};
use std::io::Write;

extern crate lzf_headers;

fn main() {
    let mut v_buffer = Vec::new();
    io::stdin().read_to_end(&mut v_buffer).unwrap();
    let _result = match lzf_headers::decompress_lzf(&v_buffer) {
        Ok(decompressed) => io::stdout().write(&decompressed),
        Err(why) => panic!("Could not decompress body: {}", why)
    };
}
