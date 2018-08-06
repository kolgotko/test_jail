extern crate libc;

use libc::*;
use libc::iovec;
use std::ffi::CString;
use std::collections::HashMap;
use std::mem::size_of;


#[derive(Debug)]
enum PreIov {
    String(String),
    Int(i32),
}

struct JailTool {}

impl JailTool {

    fn map_to_iovec_slice(&self, mut map: HashMap<&str, PreIov>) {

        for (key, value) in map.drain() {

            let c_key = CString::new(key.to_string()).unwrap();
            let iovec_key = libc::iovec {
                iov_base: c_key.as_ptr() as *mut _,
                iov_len: c_key.as_bytes_with_nul().len(),
            };

            match value {

                PreIov::String(s) => {

                    println!("{:?}", s);
                    let c_string = CString::new(s).unwrap();
                    let iovec = libc::iovec {
                        iov_base: c_string.as_ptr() as *mut _,
                        iov_len: c_string.as_bytes_with_nul().len(),
                    };

                },
                PreIov::Int(i) => {

                    let iovec = libc::iovec {
                        iov_base: &i as *const _ as *mut _,
                        iov_len: size_of::<i32>(),
                    };

                },

            }

        }

    }

}

fn main() {

    let mut map = HashMap::new();
    map.insert("jid", PreIov::Int(2));
    map.insert("path", PreIov::String("/path/to/dest".to_string()));

    // for (key, value) in map.iter() {

    //     println!("{:?}", key);
    //     println!("{:?}", value);

    // }

    let jail_tool = JailTool {};
    jail_tool.map_to_iovec_slice(map);

}
