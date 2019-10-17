//
// filter wget output, show only progress
//
// Usage:
//      function wg() { f=$(echo $(basename $@) | cut -d'?' -f1); wget --progress=bar:force --no-check-certificate "$@" -O "$f" 2>&1 | wget_filter; }
//      cargo build && wget --progress=bar:force http://ipv4.download.thinkbroadband.com/5MB.zip -O /tmp/tmp.zip 2>&1 | target/debug/wget_filter
//

use std::io;
use std::io::prelude::*;
use std::str;

enum Status {
    Ready,
    FoundCr,
    End,
}

fn main() {
    let mut buffer = [0; 1024];
    let mut process = vec![];
    let mut status = Status::Ready;

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

        for &c in buffer[..n].iter() {
            handle_char(&mut status, c, &mut process);

            match status {
                Status::End => return,
                _ => {}
            }
        }
    }
}

fn handle_char(status: &mut Status, c: u8, process: &mut std::vec::Vec<u8>) {
    match status {
        Status::Ready => {
            if c == 0x0D {
                *status = Status::FoundCr;
                process.push(c);
            }
        }
        Status::FoundCr => {
            if c == 0x0D {
                print_process(process);
                process.clear();
                process.push(c);
            } else if c == 0x0A {
                *status = Status::End;
                process.push(c);
                print_process(process);
            } else {
                process.push(c);
            }
        }
        _ => {}
    }
}

fn print_process(buf: &[u8]) {
    match str::from_utf8(buf) {
        Ok(v) => {
            print!("{}", v);
            io::stdout().flush().unwrap();
        }
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
}
