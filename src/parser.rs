use std::ffi::CString;
use std::os::raw::*;
use std::ptr;

use super::sys::*;
use super::tree::*;
use super::utils::*;

pub struct Parser {
  inner: *mut c_void,
  input: *const i8,
}

impl Drop for Parser {
  fn drop(&mut self) {
    self.free_input();
    unsafe {
      cabocha_destroy(self.inner);
    }
  }
}

impl Parser {
  pub fn new<T: Into<Vec<u8>>>(arg: T) -> Parser {
    Parser {
      inner: unsafe { cabocha_new2(str_to_heap_ptr(arg)) } as *mut c_void,
      input: ptr::null(),
    }
  }

  fn free_input(&mut self) {
    if !self.input.is_null() {
      unsafe {
        CString::from_raw(self.input as *mut i8);
      }
    }
  }

  pub fn parse_to_tree<T: Into<Vec<u8>>>(&mut self, text: T) -> Tree {
    let tree_ptr = unsafe { cabocha_tree_new() } as *mut c_void;
    let mut tree = Tree::new_from_ptr(tree_ptr);
    tree.set_sentence(text);
    unsafe { cabocha_parse_tree(self.inner, tree.inner) };
    tree
  }

  pub fn parse_to_str<T: Into<Vec<u8>>>(&mut self, text: T) -> String {
    self.free_input();
    self.input = str_to_heap_ptr(text.into());
    unsafe { ptr_to_string(cabocha_sparse_tostr(self.inner, self.input)) }
  }

  pub fn get_last_error(&self) -> String {
    unsafe { ptr_to_string(cabocha_strerror(self.inner)) }
  }
}
