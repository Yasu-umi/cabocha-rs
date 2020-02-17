pub const CABOCHA_EUC_JP: i32 = 0;
pub const CABOCHA_CP932: i32 = 1;
pub const CABOCHA_UTF8: i32 = 2;
pub const CABOCHA_ASCII: i32 = 3;

pub const CABOCHA_IPA: i32 = 0;
pub const CABOCHA_JUMAN: i32 = 1;
pub const CABOCHA_UNIDIC: i32 = 2;

pub const CABOCHA_FORMAT_TREE: i32 = 0;
pub const CABOCHA_FORMAT_LATTICE: i32 = 1;
pub const CABOCHA_FORMAT_TREE_LATTICE: i32 = 2;
pub const CABOCHA_FORMAT_XML: i32 = 3;
pub const CABOCHA_FORMAT_CONLL: i32 = 4;
pub const CABOCHA_FORMAT_NONE: i32 = 5;

pub const CABOCHA_INPUT_RAW_SENTENCE: i32 = 0;
pub const CABOCHA_INPUT_POS: i32 = 1;
pub const CABOCHA_INPUT_CHUNK: i32 = 2;
pub const CABOCHA_INPUT_SELECTION: i32 = 3;
pub const CABOCHA_INPUT_DEP: i32 = 4;

pub const CABOCHA_OUTPUT_RAW_SENTENCE: i32 = 0;
pub const CABOCHA_OUTPUT_POS: i32 = 1;
pub const CABOCHA_OUTPUT_CHUNK: i32 = 2;
pub const CABOCHA_OUTPUT_SELECTION: i32 = 3;
pub const CABOCHA_OUTPUT_DEP: i32 = 4;

#[allow(dead_code)]
pub const CABOCHA_TRAIN_NE: i32 = 0;
#[allow(dead_code)]
pub const CABOCHA_TRAIN_CHUNK: i32 = 1;
#[allow(dead_code)]
pub const CABOCHA_TRAIN_DEP: i32 = 2;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum CABOCHA_CHARSET_TYPE {
  EUC_JP = CABOCHA_EUC_JP as isize,
  CP932 = CABOCHA_CP932 as isize,
  UTF8 = CABOCHA_UTF8 as isize,
  ASCII = CABOCHA_ASCII as isize,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum CABOCHA_POSSET_TYPE {
  IPA = CABOCHA_IPA as isize,
  JUMAN = CABOCHA_JUMAN as isize,
  UNIDIC = CABOCHA_UNIDIC as isize,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum CABOCHA_FORMAT {
  TREE = CABOCHA_FORMAT_TREE as isize,
  LATTICE = CABOCHA_FORMAT_LATTICE as isize,
  TREE_LATTICE = CABOCHA_FORMAT_TREE_LATTICE as isize,
  XML = CABOCHA_FORMAT_XML as isize,
  CONLL = CABOCHA_FORMAT_CONLL as isize,
  NONE = CABOCHA_FORMAT_NONE as isize,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum CABOCHA_INPUT {
  RAW_SENTENCE = CABOCHA_INPUT_RAW_SENTENCE as isize,
  POS = CABOCHA_INPUT_POS as isize,
  CHUNK = CABOCHA_INPUT_CHUNK as isize,
  SELECTION = CABOCHA_INPUT_SELECTION as isize,
  DEP = CABOCHA_INPUT_DEP as isize,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum CABOCHA_OUTPUT {
  RAW_SENTENCE = CABOCHA_OUTPUT_RAW_SENTENCE as isize,
  POS = CABOCHA_OUTPUT_POS as isize,
  CHUNK = CABOCHA_OUTPUT_CHUNK as isize,
  SELECTION = CABOCHA_OUTPUT_SELECTION as isize,
  DEP = CABOCHA_OUTPUT_DEP as isize,
}
