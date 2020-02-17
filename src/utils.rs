use std::ffi::{CStr, CString};
use std::os::raw::*;
use std::{slice, str};

pub fn ptr_to_string(ptr: *const c_char) -> String {
  let strs = if ptr.is_null() {
    ""
  } else {
    let cstr = unsafe { CStr::from_ptr(ptr) };
    if let Ok(s) = str::from_utf8(cstr.to_bytes()) {
      s
    } else {
      ""
    }
  };
  strs.to_string()
}

pub fn ptr_to_vec_string(ptr: *const *const c_char, size: usize) -> Vec<String> {
  let slice = unsafe { slice::from_raw_parts(ptr, size) };
  slice
    .iter()
    .map(|raw_str| ptr_to_string(*raw_str))
    .collect()
}

pub fn str_to_heap_ptr<T: Into<Vec<u8>>>(input: T) -> *mut c_char {
  CString::new(input).unwrap().into_raw()
}
