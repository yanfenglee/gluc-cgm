use sha1::{Sha1, Digest};
use core::fmt::Write;

pub fn sha1(data: &String) -> String {

    // create a Sha1 object
    let mut sh = Sha1::default();

    // write input message
    sh.input(data.as_bytes());

    let output = &sh.result()[..];

    let mut hex_str = String::with_capacity(2 * output.len());
    for byte in output {
        write!(hex_str, "{:02x}", byte).expect("sha1 error");
    }

    hex_str
}

#[cfg(test)]
mod test {

    // sh.input(b"hello world");
    // assert_eq!(output[..], [0x2a, 0xae, 0x6c, 0x35, 0xc9, 0x4f, 0xcf, 0xb4, 0x15, 0xdb,
    //    0xe9, 0x5f, 0x40, 0x8b, 0x9c, 0xe9, 0x1e, 0xe8, 0x46, 0xed]);

    use crate::util::hash;

    #[test]
    fn test_sha1() {
        let res = hash::sha1(&"hello world".into());
        println!("{}", res);
    }
}
