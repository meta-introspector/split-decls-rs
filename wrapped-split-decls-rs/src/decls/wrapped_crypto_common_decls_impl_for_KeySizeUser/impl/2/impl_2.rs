use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > KeySizeUser for T where T : InnerUser , T :: Inner : KeySizeUser , { type KeySize = < T :: Inner as KeySizeUser > :: KeySize ; }
}