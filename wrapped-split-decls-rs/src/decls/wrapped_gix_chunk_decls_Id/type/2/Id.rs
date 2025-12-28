use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " An identifier to describe the kind of chunk, unique within a chunk file, typically in ASCII"] pub type Id = [u8 ; 4] ;
}