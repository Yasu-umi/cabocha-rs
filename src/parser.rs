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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::consts::*;
  use std::cmp::Ordering;

  macro_rules! assert_eq_approx {
    ($left:expr, $right:expr, $tol: expr) => {
      match (&$left, &$right, $tol) {
        (left_val, right_val, tol_val) => {
          let delta = (left_val - right_val).abs();
          let b = match delta.partial_cmp(&tol_val) {
            None | Some(Ordering::Greater) => true,
            _ => false,
          };
          if b {
            panic!(
              "assertion failed: `(left ≈ right)` \
               (left: `{:?}`, right: `{:?}`) \
               with ∆={:1.1e} (allowed ∆={:e})",
              left_val, right_val, delta, tol_val
            )
          }
        }
      }
    };
  }

  #[test]
  fn test_parse_to_str() {
    let mut parser = Parser::new("");
    assert_eq!(
      r#"  一郎は---------D
    二郎が-D     |
      描いた-D   |
          絵を---D
          三郎に-D
          贈った。
EOS
"#,
      parser.parse_to_str("一郎は二郎が描いた絵を三郎に贈った。")
    );
  }

  #[test]
  fn test_parse_to_tree_to_string() {
    let mut parser = Parser::new("");
    let tree = parser.parse_to_tree("一郎は二郎が描いた絵を三郎に贈った。");
    let result = r#"* 0 5D 0/1 -1.663431
一郎	名詞,固有名詞,人名,名,*,*,一郎,イチロウ,イチロー
は	助詞,係助詞,*,*,*,*,は,ハ,ワ
* 1 2D 0/1 0.916559
二郎	名詞,固有名詞,人名,名,*,*,二郎,ジロウ,ジロー
が	助詞,格助詞,一般,*,*,*,が,ガ,ガ
* 2 3D 0/1 1.678682
描い	動詞,自立,*,*,五段・カ行イ音便,連用タ接続,描く,エガイ,エガイ
た	助動詞,*,*,*,特殊・タ,基本形,た,タ,タ
* 3 5D 0/1 -1.663431
絵	名詞,一般,*,*,*,*,絵,エ,エ
を	助詞,格助詞,一般,*,*,*,を,ヲ,ヲ
* 4 5D 0/1 -1.663431
三郎	名詞,固有名詞,地域,一般,*,*,三郎,サブロウ,サブロー
に	助詞,格助詞,一般,*,*,*,に,ニ,ニ
* 5 -1D 0/1 0.000000
贈っ	動詞,自立,*,*,五段・ラ行,連用タ接続,贈る,オクッ,オクッ
た	助動詞,*,*,*,特殊・タ,基本形,た,タ,タ
。	記号,句点,*,*,*,*,。,。,。
EOS
"#;

    assert_eq!(result, tree.to_string(CABOCHA_FORMAT::LATTICE));
  }

  // for macro https://github.com/rust-lang/rust-clippy/issues/3900
  #[allow(clippy::cognitive_complexity)]
  #[test]
  fn test_parse_to_tree_chunks() {
    let mut parser = Parser::new("");
    let tree = parser.parse_to_tree("一郎は二郎が描いた絵を三郎に贈った。");
    let chunks = tree.chunks();
    assert_eq!(6, chunks.len());

    assert_eq!(5, chunks[0].link());
    assert_eq!(0, chunks[0].head_pos());
    assert_eq!(1, chunks[0].func_pos());
    assert_eq_approx!(-1.663_430_8, chunks[0].score(), 4.0e-7);
    assert_eq!("", chunks[0].additional_info());

    assert_eq!(2, chunks[1].link());
    assert_eq!(0, chunks[1].head_pos());
    assert_eq!(1, chunks[1].func_pos());
    assert_eq_approx!(0.916_559, chunks[1].score(), 4.0e-7);
    assert_eq!("", chunks[1].additional_info());

    assert_eq!(3, chunks[2].link());
    assert_eq!(0, chunks[2].head_pos());
    assert_eq!(1, chunks[2].func_pos());
    assert_eq_approx!(1.678_682, chunks[2].score(), 4.0e-7);
    assert_eq!("", chunks[2].additional_info());

    assert_eq!(5, chunks[3].link());
    assert_eq!(0, chunks[3].head_pos());
    assert_eq!(1, chunks[3].func_pos());
    assert_eq_approx!(-1.663_431, chunks[3].score(), 4.0e-7);
    assert_eq!("", chunks[3].additional_info());

    assert_eq!(5, chunks[4].link());
    assert_eq!(0, chunks[4].head_pos());
    assert_eq!(1, chunks[4].func_pos());
    assert_eq_approx!(-1.663_431, chunks[4].score(), 4.0e-7);
    assert_eq!("", chunks[4].additional_info());

    assert_eq!(-1, chunks[5].link());
    assert_eq!(0, chunks[5].head_pos());
    assert_eq!(1, chunks[5].func_pos());
    assert_eq_approx!(0.0, chunks[5].score(), 4.0e-7);
    assert_eq!(
      vec![
        "GPUNC:。",
        "FPUNC:。",
        "FHS:贈っ",
        "FHP0:動詞",
        "FHP1:自立",
        "FHF:連用タ接続",
        "FFS:た",
        "FFP0:助動詞",
        "FFF:基本形",
        "FLS:贈っ",
        "FLP0:動詞",
        "FLP1:自立",
        "FLF:連用タ接続",
        "FRS:。",
        "FRP0:記号",
        "FRP1:句点",
        "LF:た",
        "RL:贈っ",
        "RH:贈っ",
        "RF:た",
        "FEOS:1",
        "A:基本形"
      ],
      chunks[5].feature_list()
    );
    assert_eq!(22, chunks[5].feature_list_size());
    assert_eq!("", chunks[5].additional_info());
  }

  #[test]
  fn test_parse_to_tree_sentence() {
    let mut parser = Parser::new("");
    let tree = parser.parse_to_tree("一郎は二郎が描いた絵を三郎に贈った。");
    assert_eq!("一郎は二郎が描いた絵を三郎に贈った。", tree.sentence());
  }

  #[test]
  fn test_parse_to_tree_tokens() {
    let mut parser = Parser::new("");
    let tree = parser.parse_to_tree("一郎は二郎が描いた絵を三郎に贈った。");
    let tokens = tree.tokens();
    assert_eq!(13, tokens.len());

    assert_eq!("一郎", tokens[0].surface());
    assert_eq!("一郎", tokens[0].normalized_surface());
    let feature_list = vec![
      "名詞",
      "固有名詞",
      "人名",
      "名",
      "*",
      "*",
      "一郎",
      "イチロウ",
      "イチロー",
    ];
    assert_eq!(feature_list.join(","), tokens[0].feature());
    assert_eq!(feature_list, tokens[0].feature_list());
    assert_eq!(9, tokens[0].feature_list_size());
    assert_eq!("", tokens[0].ne());
    assert_eq!("", tokens[0].additional_info());

    assert_eq!("は", tokens[1].surface());
    assert_eq!("は", tokens[1].normalized_surface());
    let feature_list = vec!["助詞", "係助詞", "*", "*", "*", "*", "は", "ハ", "ワ"];
    assert_eq!(feature_list.join(","), tokens[1].feature());
    assert_eq!(feature_list, tokens[1].feature_list());
    assert_eq!(9, tokens[1].feature_list_size());
    assert_eq!("", tokens[1].ne());
    assert_eq!("", tokens[1].additional_info());

    assert_eq!("二郎", tokens[2].surface());
    assert_eq!("二郎", tokens[2].normalized_surface());
    let feature_list = vec![
      "名詞",
      "固有名詞",
      "人名",
      "名",
      "*",
      "*",
      "二郎",
      "ジロウ",
      "ジロー",
    ];
    assert_eq!(feature_list.join(","), tokens[2].feature());
    assert_eq!(feature_list, tokens[2].feature_list());
    assert_eq!(9, tokens[2].feature_list_size());
    assert_eq!("", tokens[2].ne());
    assert_eq!("", tokens[2].additional_info());
  }
}
