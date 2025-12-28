use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A version of [`IoSliceMut`] that allows the buffer to be uninitialised."] # [doc = ""] # [doc = " [`IoSliceMut`]: std::io::IoSliceMut"] # [repr (transparent)] pub struct MaybeUninitSlice < 'a > (sys :: MaybeUninitSlice < 'a >) ;
}