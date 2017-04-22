use std::os::raw::*;
use std::ffi::{CStr, CString};
use std::str;


pub fn ptr_to_string(ptr: *const c_char) -> String {
    unsafe {
        str::from_utf8(CStr::from_ptr(ptr).to_bytes()).map_err(|_| "").unwrap().to_string()
    }
}

pub fn ptr_to_vec_string(ptr: *const *const c_char, size: usize) -> Vec<String> {
    let vec: Vec<*const c_char>;
    unsafe {
        vec = Vec::from_raw_parts(ptr as *mut *const c_char, size, size);
    }
    vec.iter().map(|raw_str| ptr_to_string(*raw_str)).collect()
}

pub fn str_to_heap_ptr<T: Into<Vec<u8>>>(input: T) -> *mut i8 {
    CString::new(input).unwrap().into_raw()
}
