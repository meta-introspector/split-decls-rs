use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Fundamental unit of memory access and layout."] # [derive (Copy , Clone , PartialEq , Eq , Hash , Debug)] # [cfg_attr (feature = "nightly" , derive (HashStable_Generic))] pub enum Primitive { # [doc = " The `bool` is the signedness of the `Integer` type."] # [doc = ""] # [doc = " One would think we would not care about such details this low down,"] # [doc = " but some ABIs are described in terms of C types and ISAs where the"] # [doc = " integer arithmetic is done on {sign,zero}-extended registers, e.g."] # [doc = " a negative integer passed by zero-extension will appear positive in"] # [doc = " the callee, and most operations on it will produce the wrong values."] Int (Integer , bool) , Float (Float) , Pointer (AddressSpace) , }
}