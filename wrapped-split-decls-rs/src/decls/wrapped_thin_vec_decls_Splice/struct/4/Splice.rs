use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A splicing iterator for `ThinVec`."] # [doc = ""] # [doc = " This struct is created by [`ThinVec::splice`][]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use thin_vec::thin_vec;"] # [doc = ""] # [doc = " let mut v = thin_vec![0, 1, 2];"] # [doc = " let new = [7, 8];"] # [doc = " let iter: thin_vec::Splice<_> = v.splice(1.., new);"] # [doc = " ```"] # [derive (Debug)] pub struct Splice < 'a , I : Iterator + 'a > { drain : Drain < 'a , I :: Item > , replace_with : I , }
}