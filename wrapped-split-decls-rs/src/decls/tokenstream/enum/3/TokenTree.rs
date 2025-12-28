use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Part of a `TokenStream`."] # [derive (Debug , Clone , PartialEq , Encodable , Decodable , HashStable_Generic)] pub enum TokenTree { # [doc = " A single token. Should never be `OpenDelim` or `CloseDelim`, because"] # [doc = " delimiters are implicitly represented by `Delimited`."] Token (Token , Spacing) , # [doc = " A delimited sequence of token trees."] Delimited (DelimSpan , DelimSpacing , Delimiter , TokenStream) , }
}