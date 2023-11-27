#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#![feature(asm_const)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

use std::cmp::{max, min};
use std::ops::Index;
#[cfg(feature = "axstd")]
use axstd::println;

use std::vec::Vec;

extern crate arceos_api;

const PLASH_START: usize = 0x22000000;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let apps_start = PLASH_START as *const u8;
    //let apps_size = 32; // Dangerous!!! We need to get accurate size of apps.

    println!("Load payload ...");


    let head = read_head(PLASH_START);
    let app_num = head.app_num;
    println!("app num: {}", app_num);

    register_abi(SYS_HELLO, abi_hello as usize);
    register_abi(SYS_PUTCHAR, abi_putchar as usize);
    register_abi(SYS_TERMINATE, abi_terminate as usize);

    let mut app_start = head.start;

    const RUN_START: usize = 0xffff_ffc0_8010_0000;
    for i in 0..app_num {
        let app_size =  head.apps_size.get(i).unwrap().clone();
        println!("app data size: {}", app_size);

        let run_code = unsafe {
            core::slice::from_raw_parts_mut(RUN_START as *mut u8, app_size)
        };

        let load_code = unsafe { core::slice::from_raw_parts(app_start as *const u8, app_size) };

        run_code.copy_from_slice(load_code );
        println!("run code {:?}; address [{:?}]", run_code, run_code.as_ptr());
        // let code = unsafe { core::slice::from_raw_parts(app_start as *const u8, app_size) };
        // println!("content: {:?}", &code[..app_size]);

        println!("Execute app ...");
        let arg0: u8 = b'A';

        // execute app
        unsafe { core::arch::asm!("
        li      t0, {abi_num}
        slli    t0, t0, 3
        la      t1, {abi_table}
        add     t1, t1, t0
        ld      t1, (t1)
        jalr    t1
        li      t2, {run_start}
        jalr    t2
        j       .",
        run_start = const RUN_START,
        abi_table = sym ABI_TABLE,
        //abi_num = const SYS_HELLO,
        abi_num = const SYS_TERMINATE,
        in("a0") arg0,
        )}


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

const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_TERMINATE: usize = 3;

static mut ABI_TABLE: [usize; 16] = [0; 16];

fn register_abi(num: usize, handle: usize) {
    unsafe { ABI_TABLE[num] = handle; }
}

fn abi_hello() {
    println!("[ABI:Hello] Hello, Apps!");
}

fn abi_putchar(c: char) {
    println!("[ABI:Print] {c}");
}

fn abi_terminate(c: char) {
    println!("terminate");
    arceos_api::sys::ax_terminate();
}