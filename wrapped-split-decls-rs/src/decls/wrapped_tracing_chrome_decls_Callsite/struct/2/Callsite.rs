use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct Callsite { tid : usize , name : String , target : String , file : Option < & 'static str > , line : Option < u32 > , args : Option < Arc < Object > > , }
}