use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A [`FileId`] and [`Edition`] bundled up together."] # [doc = " The MSB is reserved for `HirFileId` encoding, more upper bits are used to then encode the edition."] # [derive (Clone , Copy , PartialEq , Eq , Hash , PartialOrd , Ord)] pub struct EditionedFileId (u32) ;