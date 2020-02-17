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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ptr_to_vec_string() {
    let vec_string = vec![String::from("string1"), String::from("string2")];
    let vec_cstring = vec_string
      .iter()
      .map(|arg| CString::new(arg.as_str()).unwrap())
      .collect::<Vec<CString>>();
    let mut vec_ptr = vec_cstring
      .iter()
      .map(|arg| arg.as_ptr())
      .collect::<Vec<*const c_char>>();
    vec_ptr.push(std::ptr::null());
    let ptr: *const *const c_char = vec_ptr.as_ptr();
    let result = ptr_to_vec_string(ptr, vec_string.len());
    assert_eq!(vec_string, result);
  }

  #[test]
  fn test_str_to_heap_ptr_and_ptr_to_string() {
    let string = "string";
    let ptr = str_to_heap_ptr(string);
    let result = ptr_to_string(ptr);
    assert_eq!(string, result);
    unsafe {
      CString::from_raw(ptr);
    }
  }
}
