use std::ffi::CString;
use std::os::raw::*;
use std::ptr;

use super::chunk::*;
use super::consts::*;
use super::sys::*;
use super::token::*;
use super::utils::*;

pub struct Tree {
  pub inner: *mut c_void,
  input: *const i8,
}

impl Drop for Tree {
  fn drop(&mut self) {
    self.free_input();
    unsafe {
      cabocha_tree_destroy(self.inner);
    }
  }
}

impl Default for Tree {
  fn default() -> Self {
    Self::new()
  }
}

impl Tree {
  pub fn new_from_ptr(inner: *mut c_void) -> Tree {
    Tree {
      inner,
      input: ptr::null(),
    }
  }

  fn free_input(&self) {
    if !self.input.is_null() {
      unsafe {
        CString::from_raw(self.input as *mut i8);
      }
    }
  }

  pub fn new() -> Tree {
    Tree {
      inner: unsafe { cabocha_tree_new() } as *mut c_void,
      input: ptr::null(),
    }
  }

  pub fn sentence(&self) -> String {
    unsafe { ptr_to_string(cabocha_tree_sentence(self.inner)) }
  }

  pub fn sentence_size(&self) -> usize {
    unsafe { cabocha_tree_sentence_size(self.inner) }
  }

  pub fn set_sentence<T: Into<Vec<u8>>>(&mut self, sentence: T) {
    let string = sentence.into();
    let len = string.len();
    self.free_input();
    self.input = str_to_heap_ptr(string);
    unsafe {
      cabocha_tree_set_sentence(self.inner, self.input, len);
    }
  }

  pub fn token(&self, index: usize) -> Option<Token> {
    let raw_ptr = unsafe { cabocha_tree_token(self.inner, index) };
    if raw_ptr.is_null() {
      None
    } else {
      Some(Token::new(raw_ptr))
    }
  }

  pub fn chunk(&self, index: usize) -> Option<Chunk> {
    let raw_ptr = unsafe { cabocha_tree_chunk(self.inner, index) };
    if raw_ptr.is_null() {
      None
    } else {
      Some(Chunk::new(raw_ptr))
    }
  }

  pub fn add_token(&self) -> Option<Token> {
    let raw_ptr = unsafe { cabocha_tree_add_token(self.inner) };
    if raw_ptr.is_null() {
      None
    } else {
      Some(Token::new(raw_ptr))
    }
  }

  pub fn add_chunk(&self) -> Option<Chunk> {
    let raw_ptr = unsafe { cabocha_tree_add_chunk(self.inner) };
    if raw_ptr.is_null() {
      None
    } else {
      Some(Chunk::new(raw_ptr))
    }
  }

  pub fn read(&self, input_layer: CABOCHA_INPUT) -> bool {
    let len = ptr_to_string(self.input).len();
    unsafe { cabocha_tree_read(self.inner, self.input, len, input_layer as i32) != 0 }
  }

  pub fn empty(&self) -> bool {
    unsafe { cabocha_tree_empty(self.inner) != 0 }
  }

  pub fn clear(&self) {
    unsafe {
      cabocha_tree_clear(self.inner);
    }
    self.free_input();
  }

  pub fn clear_chunk(&self) {
    unsafe { cabocha_tree_clear_chunk(self.inner) }
    self.free_input();
  }

  pub fn chunk_size(&self) -> usize {
    unsafe { cabocha_tree_chunk_size(self.inner) }
  }

  pub fn token_size(&self) -> usize {
    unsafe { cabocha_tree_token_size(self.inner) }
  }

  pub fn size(&self) -> usize {
    unsafe { cabocha_tree_size(self.inner) }
  }

  pub fn to_string(&self, format_type: CABOCHA_FORMAT) -> String {
    unsafe { ptr_to_string(cabocha_tree_tostr(self.inner, format_type as c_int)) }
  }

  pub fn charset(&self) -> Option<CABOCHA_CHARSET_TYPE> {
    let val = unsafe { cabocha_tree_charset(self.inner) };
    match val {
      CABOCHA_EUC_JP => Some(CABOCHA_CHARSET_TYPE::EUC_JP),
      CABOCHA_CP932 => Some(CABOCHA_CHARSET_TYPE::CP932),
      CABOCHA_UTF8 => Some(CABOCHA_CHARSET_TYPE::UTF8),
      CABOCHA_ASCII => Some(CABOCHA_CHARSET_TYPE::ASCII),
      _ => None,
    }
  }

  pub fn set_charset(&self, charset: CABOCHA_CHARSET_TYPE) {
    unsafe { cabocha_tree_set_charset(self.inner, charset as c_int) }
  }

  pub fn posset(&self) -> Option<CABOCHA_POSSET_TYPE> {
    let val = unsafe { cabocha_tree_posset(self.inner) };
    match val {
      CABOCHA_IPA => Some(CABOCHA_POSSET_TYPE::IPA),
      CABOCHA_JUMAN => Some(CABOCHA_POSSET_TYPE::JUMAN),
      CABOCHA_UNIDIC => Some(CABOCHA_POSSET_TYPE::UNIDIC),
      _ => None,
    }
  }

  pub fn set_posset(&self, posset: CABOCHA_POSSET_TYPE) {
    unsafe { cabocha_tree_set_posset(self.inner, posset as c_int) }
  }

  pub fn output_layer(&self) -> Option<CABOCHA_OUTPUT> {
    let val = unsafe { cabocha_tree_output_layer(self.inner) };
    match val {
      CABOCHA_OUTPUT_POS => Some(CABOCHA_OUTPUT::POS),
      CABOCHA_OUTPUT_CHUNK => Some(CABOCHA_OUTPUT::CHUNK),
      CABOCHA_OUTPUT_SELECTION => Some(CABOCHA_OUTPUT::SELECTION),
      CABOCHA_OUTPUT_DEP => Some(CABOCHA_OUTPUT::DEP),
      _ => None,
    }
  }

  pub fn set_output_layer(&self, output_layer: CABOCHA_OUTPUT) {
    unsafe { cabocha_tree_set_output_layer(self.inner, output_layer as c_int) }
  }

  pub fn tokens(&self) -> Vec<Token> {
    self.token_iter().collect()
  }

  pub fn chunks(&self) -> Vec<Chunk> {
    self.chunk_iter().collect()
  }

  pub fn chunk_iter(&self) -> ChunkIter {
    ChunkIter::new(&self)
  }

  pub fn token_iter(&self) -> TokenIter {
    TokenIter::new(&self)
  }
}
