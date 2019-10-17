//
// filter wget output, show only progress
//
// Usage:
//      function wg() { f=$(echo $(basename $@) | cut -d'?' -f1); wget --progress=bar:force --no-check-certificate "$@" -O "$f" 2>&1 | wget_filter; }
//      cargo build && wget --progress=bar:force http://ipv4.download.thinkbroadband.com/5MB.zip -O ttt 2>&1 | target/debug/wget_filter
//

use std::io;
use std::io::prelude::*;
use std::str;

fn main() {
    let mut buffer = [0; 1024];

    loop {
        let n = match io::stdin().read(&mut buffer[..]) {
            Ok(len) => {
                if len == 0 {
                    return;
                }

                len
            }
            Err(error) => {
                eprintln!("error: {}", error);
                return;
            }
        };

        handle_buffer(&buffer[..n])
    }
}

fn handle_buffer(buf: &[u8]) {
    let mut v = vec![];

    for (i, c) in buf.iter().enumerate() {
        if *c == 0x0A || *c == 0x0D {
            v.push(i);
        }
    }

    println!("buf = {:?}, {}", buf, buf.len());
    println!("v = {:?}\n", v);

    // if let Some(0x0D) = buf.get(0) {
    //     match str::from_utf8(buf) {
    //         Ok(v) => {
    //             print!("{}", v);
    //             // print!("{:?}\n", v.lines());
    //             io::stdout().flush().unwrap();
    //         }
    //         Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    //     };
    // }
}
