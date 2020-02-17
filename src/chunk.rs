use super::sys::*;
use super::tree::*;
use super::utils::*;

#[derive(Debug, Clone)]
pub struct Chunk {
  self_ptr: *const cabocha_chunk_t,
}

impl Chunk {
  pub fn new(raw_ptr: *const cabocha_chunk_t) -> Chunk {
    Chunk { self_ptr: raw_ptr }
  }

  pub fn link(&self) -> i32 {
    unsafe { (*self.self_ptr).link }
  }

  pub fn head_pos(&self) -> usize {
    unsafe { (*self.self_ptr).head_pos }
  }

  pub fn func_pos(&self) -> usize {
    unsafe { (*self.self_ptr).func_pos }
  }

  pub fn token_size(&self) -> usize {
    unsafe { (*self.self_ptr).token_size }
  }

  pub fn token_pos(&self) -> usize {
    unsafe { (*self.self_ptr).token_pos }
  }

  pub fn score(&self) -> f32 {
    unsafe { (*self.self_ptr).score }
  }

  pub fn feature_list(&self) -> Vec<String> {
    unsafe {
      let chunk = &*self.self_ptr;
      ptr_to_vec_string(chunk.feature_list, chunk.feature_list_size as usize)
    }
  }

  pub fn additional_info(&self) -> String {
    unsafe { ptr_to_string((*self.self_ptr).additional_info) }
  }

  pub fn feature_list_size(&self) -> u16 {
    unsafe { (*self.self_ptr).feature_list_size }
  }
}

pub struct ChunkIter<'a> {
  tree: &'a Tree,
  pos: usize,
}

impl<'a> Iterator for ChunkIter<'a> {
  type Item = Chunk;

  fn next(&mut self) -> Option<Chunk> {
    let chunk = self.tree.chunk(self.pos);
    self.pos += 1;
    chunk
  }
}

impl<'a> ChunkIter<'a> {
  pub fn new(tree: &Tree) -> ChunkIter {
    ChunkIter { tree, pos: 0 }
  }
}
