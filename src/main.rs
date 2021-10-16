// main.rs

use ciborium::value::*;
use flate2::read::ZlibDecoder;
use std::io::{self, Read};

const BUFSZ: usize = 1024;

fn main() -> anyhow::Result<()> {
    let mut input = String::with_capacity(BUFSZ);
    io::stdin().read_to_string(&mut input)?;
    print!("Read {} bytes.\n", input.len());

    let mut in_buf = input.trim_end();
    if in_buf.starts_with("HC1:") {
        in_buf = &in_buf[4..];
    }

    let decode1 = base45::decode(in_buf)?;
    print!("Decoded base45 to {} bytes.\n", decode1.len());

    let mut buf = Vec::with_capacity(BUFSZ);
    let mut z = ZlibDecoder::new(decode1.as_slice());
    z.read_to_end(&mut buf)?;
    print!("Uncompressed to {} bytes.\n", buf.len());

    let cbor_dec: Value = ciborium::de::from_reader(buf.as_slice())?;
    // print!("*** ciborium:\n{:?}\n", cbor_dec);
    if let Value::Tag(tag, tag_val) = cbor_dec {
        print!("Tagged: tag {:?}\n", tag);
        if let Value::Array(val_arr) = *tag_val {
            for (i, val) in val_arr.iter().enumerate() {
                if i == 2 {
                    if let Value::Bytes(b) = val {
                        let dec2r: Result<Value, ciborium::de::Error<std::io::Error>> =
                            ciborium::de::from_reader(b.as_slice());
                        if let Ok(dec2) = dec2r {
                            print!("#2 decoded:\n{:?}\n", dec2);
                        }
                    }
                } else {
                    print!("#{} value: {:?}\n", i, val);
                }
            }
        }
    }

    Ok(())
}
// EOF
