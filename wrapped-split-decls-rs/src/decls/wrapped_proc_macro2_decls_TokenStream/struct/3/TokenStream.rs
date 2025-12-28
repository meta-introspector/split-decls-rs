use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " An abstract stream of tokens, or more concretely a sequence of token trees."] # [doc = ""] # [doc = " This type provides interfaces for iterating over token trees and for"] # [doc = " collecting token trees into one stream."] # [doc = ""] # [doc = " Token stream is both the input and output of `#[proc_macro]`,"] # [doc = " `#[proc_macro_attribute]` and `#[proc_macro_derive]` definitions."] # [derive (Clone)] pub struct TokenStream { inner : imp :: TokenStream , _marker : ProcMacroAutoTraits , }
}