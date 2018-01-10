#![feature(asm)]
use std::os::raw::{c_int, c_char};
use std::ffi::CString;

extern {
    fn strcmpeq_sse4(str1: *const c_char, str2: *const c_char) -> c_int;
    fn strcasecmpeq_sse2(str1: *const c_char, str2: *const c_char) -> c_int;
}

fn strcmpeq(str1: *const c_char, str2: *const c_char) -> c_int {
    unsafe {
        strcmpeq_sse4(str1, str2)
    }
}

fn strcasecmpeq(str1: *const c_char, str2: *const c_char) -> c_int {
    unsafe {
        strcasecmpeq_sse2(str1, str2)
    }
}

fn main() {
    
    println!("true = {}", strcmpeq(CString::new("abc").unwrap().as_ptr(),CString::new("abc").unwrap().as_ptr()));
    println!("false = {}", strcmpeq(CString::new("abc").unwrap().as_ptr(),CString::new("abd").unwrap().as_ptr()));
    println!("true = {}", strcmpeq(CString::new("abcdefghijklmnopqrstuvwxyz").unwrap().as_ptr(), CString::new("abcdefghijklmnopqrstuvwxyz").unwrap().as_ptr()));
    println!("false = {}", strcmpeq(CString::new("abcdefghijklmnopqrstuvwxyz").unwrap().as_ptr(), CString::new("abcdefghijklmnopqrstuvwxyzz").unwrap().as_ptr()));

    //my_fn();
}


