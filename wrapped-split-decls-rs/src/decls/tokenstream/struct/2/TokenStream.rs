use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A `TokenStream` is an abstract sequence of tokens, organized into [`TokenTree`]s."] # [derive (Clone , Debug , Default , Encodable , Decodable)] pub struct TokenStream (pub (crate) Arc < Vec < TokenTree > >) ;