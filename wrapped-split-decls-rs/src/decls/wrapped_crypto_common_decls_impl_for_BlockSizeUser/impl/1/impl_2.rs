use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : BlockSizeUser > BlockSizeUser for & mut T { type BlockSize = T :: BlockSize ; }
}