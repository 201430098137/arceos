#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

use std::cmp::{max, min};
use std::ops::Index;
#[cfg(feature = "axstd")]
use axstd::println;

use std::vec::Vec;

const PLASH_START: usize = 0x22000000;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let apps_start = PLASH_START as *const u8;
    //let apps_size = 32; // Dangerous!!! We need to get accurate size of apps.

    println!("Load payload ...");

    let head = read_head(PLASH_START);
    let app_num = head.app_num;
    println!("app num: {}", app_num);

    let mut app_start = head.start;

    for i in 0..app_num {
        let app_size =  head.apps_size.get(i).unwrap().clone();
        println!("app data size: {}", app_size);
        let code = unsafe { core::slice::from_raw_parts(app_start as *const u8, app_size) };
        println!("content: {:?}", &code[..app_size]);
        app_start += app_size;
    }
    println!("Load payload ok!");
}

#[inline]
fn bytes_to_usize(bytes: &[u8]) -> usize {
    usize::from_be_bytes(bytes.try_into().unwrap())
}

const  read_size:usize = 4;

struct head {
    app_num: usize,
    apps_size: Vec<usize>,
    start: usize,
}

fn read_head(start: usize) -> head {
    let mut new_data = Vec::new();
    let mut pos = start;
    loop {

        let data = unsafe { core::slice::from_raw_parts(pos as *const u8, read_size) };
        for &c in data {
            if c == b'\0' {
                let head_str =  core::str::from_utf8(new_data.as_slice()).unwrap();
                let heads:Vec<&str> = head_str.split('|').collect();
                let app_num:usize = heads[0].parse().unwrap();
                let mut apps_size:Vec<usize> = Vec::new();
                for i in 0..app_num {
                    apps_size.push(heads[i+1].parse().unwrap());
                }
                return head{
                    app_num:app_num,
                    apps_size:apps_size,
                    start: pos+1,
                };
            }
            //println!("app start: {} char {}", pos, c);
            pos += 1;
            new_data.push(c);
        }
    }
}