#![no_main]

use libfuzzer_sys::fuzz_target;

extern crate cbor;
use cbor::Decoder;

fuzz_target!(|data: &[u8]| {
    let mut d = Decoder::from_bytes(data);
    let items = d.decode().collect::<Result<_, _>>();
    let _items: Vec<(char, i32)> = match items {
        Ok(result) => result,
        Err(_error) => return,
    };
});
