use super::chunk::*;
use super::sys::*;
use super::tree::*;
use super::utils::*;

#[derive(Debug, Clone)]
pub struct Token {
  self_ptr: *const cabocha_token_t,
}

impl Token {
  pub fn new(raw_ptr: *const cabocha_token_t) -> Token {
    Token { self_ptr: raw_ptr }
  }

  pub fn surface(&self) -> String {
    ptr_to_string(unsafe { (*self.self_ptr).surface })
  }

  pub fn normalized_surface(&self) -> String {
    ptr_to_string(unsafe { (*self.self_ptr).normalized_surface })
  }

  pub fn feature(&self) -> String {
    ptr_to_string(unsafe { (*self.self_ptr).feature })
  }

  pub fn feature_list(&self) -> Vec<String> {
    unsafe {
      ptr_to_vec_string(
        (*self.self_ptr).feature_list,
        (*self.self_ptr).feature_list_size as usize,
      )
    }
  }

  pub fn feature_list_size(&self) -> u16 {
    unsafe { (*self.self_ptr).feature_list_size }
  }

  pub fn ne(&self) -> String {
    ptr_to_string(unsafe { (*self.self_ptr).ne })
  }

  pub fn additional_info(&self) -> String {
    ptr_to_string(unsafe { (*self.self_ptr).additional_info })
  }

  pub fn chunk(&self) -> Option<Chunk> {
    if self.self_ptr.is_null() {
      None
    } else {
      let chunk_ptr = unsafe { &*self.self_ptr }.chunk;
      if chunk_ptr.is_null() {
        None
      } else {
        Some(Chunk::new(chunk_ptr))
      }
    }
  }
}

pub struct TokenIter<'a> {
  tree: &'a Tree,
  pos: usize,
}

impl<'a> Iterator for TokenIter<'a> {
  type Item = Token;

  fn next(&mut self) -> Option<Token> {
    let token = self.tree.token(self.pos);
    self.pos += 1;
    token
  }
}

impl<'a> TokenIter<'a> {
  pub fn new(tree: &Tree) -> TokenIter {
    TokenIter { tree, pos: 0 }
  }
}
