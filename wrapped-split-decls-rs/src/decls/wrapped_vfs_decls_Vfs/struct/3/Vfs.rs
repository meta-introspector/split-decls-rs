use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Storage for all file changes and the file id to path mapping."] # [doc = ""] # [doc = " For more information see the [crate-level](crate) documentation."] # [derive (Default)] pub struct Vfs { interner : PathInterner , data : Vec < FileState > , changes : IndexMap < FileId , ChangedFile , BuildHasherDefault < FxHasher > > , }
}