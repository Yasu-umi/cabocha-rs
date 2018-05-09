#![allow(non_camel_case_types)]


use std::ptr;
use std::os::raw::*;
use std::ffi::CString;

use utils::*;


const CABOCHA_EUC_JP: i32 = 0;
const CABOCHA_CP932: i32 = 1;
const CABOCHA_UTF8: i32 = 2;
const CABOCHA_ASCII: i32 = 3;

const CABOCHA_IPA: i32 = 0;
const CABOCHA_JUMAN: i32 = 1;
const CABOCHA_UNIDIC: i32 = 2;

const CABOCHA_FORMAT_TREE: i32 = 0;
const CABOCHA_FORMAT_LATTICE: i32 = 1;
const CABOCHA_FORMAT_TREE_LATTICE: i32 = 2;
const CABOCHA_FORMAT_XML: i32 = 3;
const CABOCHA_FORMAT_CONLL: i32 = 4;
const CABOCHA_FORMAT_NONE: i32 = 5;

const CABOCHA_INPUT_RAW_SENTENCE: i32 = 0;
const CABOCHA_INPUT_POS: i32 = 1;
const CABOCHA_INPUT_CHUNK: i32 = 2;
const CABOCHA_INPUT_SELECTION: i32 = 3;
const CABOCHA_INPUT_DEP: i32 = 4;

const CABOCHA_OUTPUT_RAW_SENTENCE: i32 = 0;
const CABOCHA_OUTPUT_POS: i32 = 1;
const CABOCHA_OUTPUT_CHUNK: i32 = 2;
const CABOCHA_OUTPUT_SELECTION: i32 = 3;
const CABOCHA_OUTPUT_DEP: i32 = 4;

const CABOCHA_TRAIN_NE: i32 = 0;
const CABOCHA_TRAIN_CHUNK: i32 = 1;
const CABOCHA_TRAIN_DEP: i32 = 2;

#[derive(Debug)]
pub enum CABOCHA_CHARSET_TYPE {
    EUC_JP = CABOCHA_EUC_JP as isize,
    CP932 = CABOCHA_CP932 as isize,
    UTF8 = CABOCHA_UTF8 as isize,
    ASCII = CABOCHA_ASCII as isize,
}

#[derive(Debug)]
pub enum CABOCHA_POSSET_TYPE {
    IPA = CABOCHA_IPA as isize,
    JUMAN = CABOCHA_JUMAN as isize,
    UNIDIC = CABOCHA_UNIDIC as isize,
}

#[derive(Debug)]
pub enum CABOCHA_FORMAT {
    TREE = CABOCHA_FORMAT_TREE as isize,
    LATTICE = CABOCHA_FORMAT_LATTICE as isize,
    TREE_LATTICE = CABOCHA_FORMAT_TREE_LATTICE as isize,
    XML = CABOCHA_FORMAT_XML as isize,
    CONLL = CABOCHA_FORMAT_CONLL as isize,
    NONE = CABOCHA_FORMAT_NONE as isize,
}

#[derive(Debug)]
pub enum CABOCHA_INPUT {
    RAW_SENTENCE = CABOCHA_INPUT_RAW_SENTENCE as isize,
    POS = CABOCHA_INPUT_POS as isize,
    CHUNK = CABOCHA_INPUT_CHUNK as isize,
    SELECTION = CABOCHA_INPUT_SELECTION as isize,
    DEP = CABOCHA_INPUT_DEP as isize,
}

#[derive(Debug)]
pub enum CABOCHA_OUTPUT {
    RAW_SENTENCE = CABOCHA_OUTPUT_RAW_SENTENCE as isize,
    POS = CABOCHA_OUTPUT_POS as isize,
    CHUNK = CABOCHA_OUTPUT_CHUNK as isize,
    SELECTION = CABOCHA_OUTPUT_SELECTION as isize,
    DEP = CABOCHA_OUTPUT_DEP as isize,
}

#[repr(C)]
struct cabocha_chunk_t {
    link: c_int,
    head_pos: usize,
    func_pos: usize,
    token_size: usize,
    token_pos: usize,
    score: c_float,
    feature_list: *const *const c_char,
    additional_info: *const c_char,
    feature_list_size: c_ushort,
}

#[derive(Debug, Clone)]
pub struct Chunk {
    self_ptr: *const cabocha_chunk_t,
}

impl Chunk {
    fn new(raw_ptr: *const cabocha_chunk_t) -> Chunk {
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
            let chunk = &(*self.self_ptr);
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
        ChunkIter { tree: tree, pos: 0 }
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
        TokenIter { tree: tree, pos: 0 }
    }
}

#[repr(C)]
struct cabocha_token_t {
    surface: *const c_char,
    normalized_surface: *const c_char,
    feature: *const c_char,
    feature_list: *const *const c_char,
    feature_list_size: c_ushort,
    ne: *const c_char,
    additional_info: *const c_char,
    chunk: *const cabocha_chunk_t,
}

#[derive(Debug, Clone)]
pub struct Token {
    self_ptr: *const cabocha_token_t,
    chunk_ptr: *mut cabocha_chunk_t,
}

impl Token {
    fn new(raw_ptr: *const cabocha_token_t) -> Token {
        let token = unsafe { &(*raw_ptr) };
        Token {
            self_ptr: token,
            chunk_ptr: token.chunk as *mut cabocha_chunk_t,
        }
    }

    pub fn surface(&self) -> String {
        unsafe { ptr_to_string((*self.self_ptr).surface) }
    }

    pub fn normalized_surface(&self) -> String {
        unsafe { ptr_to_string((*self.self_ptr).normalized_surface) }
    }

    pub fn feature(&self) -> String {
        unsafe { ptr_to_string((*self.self_ptr).feature) }
    }

    pub fn feature_list(&self) -> Vec<String> {
        unsafe {
            let token = &(*self.self_ptr);
            ptr_to_vec_string(token.feature_list, token.feature_list_size as usize)
        }
    }

    pub fn feature_list_size(&self) -> u16 {
        unsafe { (*self.self_ptr).feature_list_size }
    }

    pub fn ne(&self) -> String {
        unsafe { ptr_to_string((*self.self_ptr).ne) }
    }

    pub fn additional_info(&self) -> String {
        unsafe { ptr_to_string((*self.self_ptr).additional_info) }
    }

    pub fn chunk(&self) -> Option<Chunk> {
        if self.chunk_ptr.is_null() {
            None
        } else {
            Some(Chunk::new(self.chunk_ptr))
        }
    }
}


#[link(name="cabocha")]
extern "C" {
    fn cabocha_do(argc: c_int, argv: *const *const c_char) -> c_int;

    /* parser */
    fn cabocha_new(argc: c_int, argv: *const *const c_char) -> *const c_void;
    fn cabocha_new2(arg: *const c_char) -> *const c_void;
    fn cabocha_strerror(cabocha: *mut c_void) -> *const c_char;
    fn cabocha_parse_tree(cabocha: *mut c_void, cabocha_tree: *mut c_void) -> *mut c_void;
    fn cabocha_sparse_tostr(cabocha: *mut c_void, str: *const c_char) -> *const c_char;
    fn cabocha_sparse_tostr2(cabocha: *mut c_void,
                             str: *const c_char,
                             length: usize)
                             -> *const c_char;
    fn cabocha_sparse_tostr3(cabocha: *mut c_void,
                             str: *const c_char,
                             length: usize,
                             output_str: *const c_char,
                             output_length: usize)
                             -> *const c_char;
    fn cabocha_destroy(cabocha: *mut c_void);
    fn cabocha_sparse_totree(cabocha: *mut c_void, str: *const c_char) -> *mut c_void;
    fn cabocha_sparse_totree2(cabocha: *mut c_void,
                              str: *const c_char,
                              length: usize)
                              -> *mut c_void;

    /* tree */
    fn cabocha_tree_new() -> *const c_void;
    fn cabocha_tree_destroy(tree: *mut c_void);
    fn cabocha_tree_empty(tree: *mut c_void) -> c_int;
    fn cabocha_tree_clear(tree: *mut c_void);
    fn cabocha_tree_clear_chunk(tree: *mut c_void);
    fn cabocha_tree_size(tree: *mut c_void) -> usize;
    fn cabocha_tree_chunk_size(tree: *mut c_void) -> usize;
    fn cabocha_tree_token_size(tree: *mut c_void) -> usize;
    fn cabocha_tree_sentence(tree: *mut c_void) -> *const c_char;
    fn cabocha_tree_sentence_size(tree: *mut c_void) -> usize;
    fn cabocha_tree_set_sentence(tree: *mut c_void, sentence: *const c_char, length: usize);
    fn cabocha_tree_read(tree: *mut c_void,
                         input: *const c_char,
                         length: usize,
                         input_layer: c_int)
                         -> c_int;
    fn cabocha_tree_read_from_mecab_node(tree: *mut c_void, node: *mut c_void) -> c_int;

    fn cabocha_tree_token(tree: *mut c_void, i: usize) -> *mut cabocha_token_t;
    fn cabocha_tree_chunk(tree: *mut c_void, i: usize) -> *mut cabocha_chunk_t;

    fn cabocha_tree_add_token(tree: *mut c_void) -> *const cabocha_token_t;
    fn cabocha_tree_add_chunk(tree: *mut c_void) -> *const cabocha_chunk_t;

    fn cabocha_tree_strdup(tree: *mut c_void, str: *const c_char) -> *const c_char;
    fn cabocha_tree_alloc(tree: *mut c_void, size: usize) -> *const c_char;

    fn cabocha_tree_tostr(tree: *mut c_void, format: c_int) -> *const c_char;
    fn cabocha_tree_tostr2(tree: *mut c_void,
                           format: c_int,
                           str: *const c_char,
                           length: usize)
                           -> *const c_char;

    fn cabocha_tree_set_charset(tree: *mut c_void, charset: c_int);
    fn cabocha_tree_charset(tree: *mut c_void) -> c_int;
    fn cabocha_tree_set_posset(tree: *mut c_void, posset: c_int);
    fn cabocha_tree_posset(tree: *mut c_void) -> c_int;
    fn cabocha_tree_set_output_layer(tree: *mut c_void, output_layer: c_int);
    fn cabocha_tree_output_layer(tree: *mut c_void) -> c_int;

    fn cabocha_learn(argx: c_int, argv: *const *const c_char) -> c_int;
    fn cabocha_system_eval(argx: c_int, argv: *const *const c_char) -> c_int;
    fn cabocha_model_index(argx: c_int, argv: *const *const c_char) -> c_int;
}

pub struct Parser {
    inner: *mut c_void,
    input: *const i8,
}

impl Drop for Parser {
    fn drop(&mut self) {
        unsafe {
            cabocha_destroy(self.inner);
            self.free_input();
        }
    }
}

impl Parser {
    pub fn new<T: Into<Vec<u8>>>(arg: T) -> Parser {
        let inner = unsafe { cabocha_new2(str_to_heap_ptr(arg)) as *mut c_void };
        Parser {
            inner: inner,
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

    pub fn parse_to_tree<T: Into<Vec<u8>>>(&mut self, text: T) -> Tree {
        let tree_ptr = unsafe { cabocha_tree_new() } as *mut c_void;
        let mut tree = Tree::new_from_ptr(tree_ptr);
        let _ = tree.set_sentence(text);
        let _ = unsafe { cabocha_parse_tree(self.inner, tree.inner) };
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

pub struct Tree {
    inner: *mut c_void,
    input: *const i8,
}

impl Drop for Tree {
    fn drop(&mut self) {
        unsafe {
            cabocha_tree_destroy(self.inner);
        }
        self.free_input();
    }
}

impl Tree {
    fn new_from_ptr(raw_ptr: *mut c_void) -> Tree {
        unsafe {
            let ref mut raw_tree = *raw_ptr;
            Tree {
                inner: raw_tree,
                input: ptr::null(),
            }
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
        unsafe {
            let inner = cabocha_tree_new() as *mut c_void;
            Tree {
                inner: inner,
                input: ptr::null(),
            }
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
