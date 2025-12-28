use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " An iterator that moves out of a vector."] # [doc = ""] # [doc = " This `struct` is created by the [`ThinVec::into_iter`][]"] # [doc = " (provided by the [`IntoIterator`] trait)."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use thin_vec::thin_vec;"] # [doc = ""] # [doc = " let v = thin_vec![0, 1, 2];"] # [doc = " let iter: thin_vec::IntoIter<_> = v.into_iter();"] # [doc = " ```"] pub struct IntoIter < T > { vec : ThinVec < T > , start : usize , }
}