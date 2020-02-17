use std::os::raw::*;

use libc::size_t;

#[repr(C)]
pub struct cabocha_token_t {
  pub surface: *const c_char,
  pub normalized_surface: *const c_char,
  pub feature: *const c_char,
  pub feature_list: *const *const c_char,
  pub feature_list_size: c_ushort,
  pub ne: *const c_char,
  pub additional_info: *const c_char,
  pub chunk: *const cabocha_chunk_t,
}

#[repr(C)]
pub struct cabocha_chunk_t {
  pub link: c_int,
  pub head_pos: size_t,
  pub func_pos: size_t,
  pub token_size: size_t,
  pub token_pos: size_t,
  pub score: c_float,
  pub feature_list: *const *const c_char,
  pub additional_info: *const c_char,
  pub feature_list_size: c_ushort,
}

#[link(name = "cabocha")]
extern "C" {
  #[allow(dead_code)]
  pub fn cabocha_do(argc: c_int, argv: *const *const c_char) -> c_int;

  /* parser */
  #[allow(dead_code)]
  pub fn cabocha_new(argc: c_int, argv: *const *const c_char) -> *const c_void;
  pub fn cabocha_new2(arg: *const c_char) -> *const c_void;
  pub fn cabocha_strerror(cabocha: *mut c_void) -> *const c_char;
  pub fn cabocha_parse_tree(cabocha: *mut c_void, cabocha_tree: *mut c_void) -> *mut c_void;
  pub fn cabocha_sparse_tostr(cabocha: *mut c_void, str: *const c_char) -> *const c_char;
  #[allow(dead_code)]
  pub fn cabocha_sparse_tostr2(
    cabocha: *mut c_void,
    str: *const c_char,
    length: size_t,
  ) -> *const c_char;
  #[allow(dead_code)]
  pub fn cabocha_sparse_tostr3(
    cabocha: *mut c_void,
    str: *const c_char,
    length: size_t,
    output_str: *const c_char,
    output_length: size_t,
  ) -> *const c_char;
  pub fn cabocha_destroy(cabocha: *mut c_void);
  #[allow(dead_code)]
  pub fn cabocha_sparse_totree(cabocha: *mut c_void, str: *const c_char) -> *mut c_void;
  #[allow(dead_code)]
  pub fn cabocha_sparse_totree2(
    cabocha: *mut c_void,
    str: *const c_char,
    length: size_t,
  ) -> *mut c_void;

  /* tree */
  pub fn cabocha_tree_new() -> *const c_void;
  pub fn cabocha_tree_destroy(tree: *mut c_void);
  pub fn cabocha_tree_empty(tree: *mut c_void) -> c_int;
  pub fn cabocha_tree_clear(tree: *mut c_void);
  pub fn cabocha_tree_clear_chunk(tree: *mut c_void);
  pub fn cabocha_tree_size(tree: *mut c_void) -> size_t;
  pub fn cabocha_tree_chunk_size(tree: *mut c_void) -> size_t;
  pub fn cabocha_tree_token_size(tree: *mut c_void) -> size_t;
  pub fn cabocha_tree_sentence(tree: *mut c_void) -> *const c_char;
  pub fn cabocha_tree_sentence_size(tree: *mut c_void) -> size_t;
  pub fn cabocha_tree_set_sentence(tree: *mut c_void, sentence: *const c_char, length: size_t);
  pub fn cabocha_tree_read(
    tree: *mut c_void,
    input: *const c_char,
    length: size_t,
    input_layer: c_int,
  ) -> c_int;
  #[allow(dead_code)]
  pub fn cabocha_tree_read_from_mecab_node(tree: *mut c_void, node: *mut c_void) -> c_int;

  pub fn cabocha_tree_token(tree: *mut c_void, i: size_t) -> *mut cabocha_token_t;
  pub fn cabocha_tree_chunk(tree: *mut c_void, i: size_t) -> *mut cabocha_chunk_t;

  pub fn cabocha_tree_add_token(tree: *mut c_void) -> *const cabocha_token_t;
  pub fn cabocha_tree_add_chunk(tree: *mut c_void) -> *const cabocha_chunk_t;

  #[allow(dead_code)]
  pub fn cabocha_tree_strdup(tree: *mut c_void, str: *const c_char) -> *const c_char;
  #[allow(dead_code)]
  pub fn cabocha_tree_alloc(tree: *mut c_void, size: size_t) -> *const c_char;

  pub fn cabocha_tree_tostr(tree: *mut c_void, format: c_int) -> *const c_char;
  #[allow(dead_code)]
  pub fn cabocha_tree_tostr2(
    tree: *mut c_void,
    format: c_int,
    str: *const c_char,
    length: size_t,
  ) -> *const c_char;

  pub fn cabocha_tree_set_charset(tree: *mut c_void, charset: c_int);
  pub fn cabocha_tree_charset(tree: *mut c_void) -> c_int;
  pub fn cabocha_tree_set_posset(tree: *mut c_void, posset: c_int);
  pub fn cabocha_tree_posset(tree: *mut c_void) -> c_int;
  pub fn cabocha_tree_set_output_layer(tree: *mut c_void, output_layer: c_int);
  pub fn cabocha_tree_output_layer(tree: *mut c_void) -> c_int;

  #[allow(dead_code)]
  pub fn cabocha_learn(argx: c_int, argv: *const *const c_char) -> c_int;
  #[allow(dead_code)]
  pub fn cabocha_system_eval(argx: c_int, argv: *const *const c_char) -> c_int;
  #[allow(dead_code)]
  pub fn cabocha_model_index(argx: c_int, argv: *const *const c_char) -> c_int;
}
