use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " When applied to functions or methods, it will turn it into a wrapper that will immediately call"] # [doc = " a de-monomorphized implementation (i.e. one that uses `&dyn Trait`)."] # [doc = ""] # [doc = " That way, the landing-pads for convenience will be as small as possible which then delegate to a single"] # [doc = " function or method for implementation."] # [doc = ""] # [doc = " The parameters using the following traits can be de-monomorphized:"] # [doc = ""] # [doc = " * `Into`"] # [doc = " * `AsRef`"] # [doc = " * `AsMut`"] # [proc_macro_attribute] pub fn momo (_attrs : TokenStream , input : TokenStream) -> TokenStream { momo :: inner (input . into ()) . into () }
}