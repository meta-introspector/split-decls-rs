use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " An owned immutable value."] # [derive (Debug , Clone)] pub struct Frozen < T > (T) ;
}