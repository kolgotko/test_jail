extern crate libc;

use libc::*;
use libc::iovec;
use std::ffi::*;
use std::collections::HashMap;


#[derive(Debug)]
enum PreIov {
    String(String),
    Int(i32),
}

struct JailTool {}

impl JailTool {

    fn map_to_iovec_slice(&self, map: HashMap<&str, PreIov>) {

        for (key, value) in map.iter() {

            let c_key = CString::new(key.to_string()).unwrap();
            let iovec_key = libc::iovec {
                iov_base: c_key.as_ptr() as *mut _,
                iov_len: c_key.as_bytes_with_nul().len(),
            };

            match value {

                PreIov::String(s) => {

                    let c_string = CString::new(s).unwrap();
                    let iovec = libc::iovec {
                        iov_base: c_string.as_ptr() as *mut _,
                        iov_len: c_string.as_bytes_with_nul().len(),
                    };

                },
                PreIov::Int(i) => {},

            }

        }

    }

}

fn main() {

    let mut map = HashMap::new();
    map.insert("jid", PreIov::Int(2));

    for (key, value) in map.iter() {

        println!("{:?}", key);
        println!("{:?}", value);

    }

}
