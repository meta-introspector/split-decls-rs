use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct ChangeFixture { pub file_position : Option < (EditionedFileId , RangeOrOffset) > , pub file_lines : Vec < usize > , pub files : Vec < EditionedFileId > , pub change : ChangeWithProcMacros , pub sysroot_files : Vec < FileId > , }
}