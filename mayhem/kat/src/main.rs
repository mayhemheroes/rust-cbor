// Additive known-answer test for the cbor crate. Asserts encode/decode round-trip
// behavior AND fixed CBOR byte-level output (RFC 7049), so a no-op patch (e.g. one
// that makes decode() return nothing, or encode() emit wrong bytes) FAILS.
// Exits non-zero on any mismatch.
extern crate cbor;

use cbor::{Decoder, Encoder};

fn s(x: &str) -> String { x.to_string() }

fn main() {
    let mut failures = 0u32;
    let mut checks = 0u32;

    macro_rules! check {
        ($cond:expr, $name:expr) => {{
            checks += 1;
            if !($cond) { eprintln!("FAIL: {}", $name); failures += 1; }
            else { println!("ok: {}", $name); }
        }};
    }

    // 1. Round-trip a list of (String, i32) pairs. `encode(&data)` emits each pair as
    //    its own top-level 2-element array; decode collects them back (crate's own example).
    let data = vec![(s("a"), 1i32), (s("b"), 2), (s("c"), 3)];
    let mut e = Encoder::from_memory();
    e.encode(&data).unwrap();
    let bytes = e.as_bytes().to_vec();
    let mut d = Decoder::from_bytes(&bytes[..]);
    let decoded: Vec<(String, i32)> =
        d.decode().collect::<Result<_, _>>().unwrap();
    check!(decoded == data, "roundtrip Vec<(String,i32)>");

    // 2. Byte-level KAT: a single small unsigned integer 1 encodes to 0x01 (RFC 7049).
    let mut e2 = Encoder::from_memory();
    e2.encode(&[1u8]).unwrap();
    check!(e2.as_bytes() == &[0x01u8], "encode u8 1 == 0x01");

    // 3. Byte-level KAT: integer 23 -> 0x17, 24 -> 0x18 0x18 (RFC 7049 major type 0).
    let mut e3 = Encoder::from_memory();
    e3.encode(&[23u8]).unwrap();
    check!(e3.as_bytes() == &[0x17u8], "encode u8 23 == 0x17");
    let mut e4 = Encoder::from_memory();
    e4.encode(&[24u8]).unwrap();
    check!(e4.as_bytes() == &[0x18u8, 0x18u8], "encode u8 24 == 0x18 0x18");

    // 4. Decode a known CBOR array [1,2,3] (0x83 0x01 0x02 0x03) back to Vec<i32>.
    let mut d2 = Decoder::from_bytes(&[0x83u8, 0x01, 0x02, 0x03][..]);
    let arr: Vec<i32> = d2.decode::<Vec<i32>>().next().unwrap().unwrap();
    check!(arr == vec![1, 2, 3], "decode 0x83010203 == [1,2,3]");

    println!("CHECKS {} FAILURES {}", checks, failures);
    if failures > 0 { std::process::exit(1); }
}
