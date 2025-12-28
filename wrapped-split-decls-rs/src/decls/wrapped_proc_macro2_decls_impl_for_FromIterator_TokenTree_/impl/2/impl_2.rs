use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Collects a number of token trees into a single stream."] impl FromIterator < TokenTree > for TokenStream { fn from_iter < I : IntoIterator < Item = TokenTree > > (tokens : I) -> Self { TokenStream :: _new (tokens . into_iter () . collect ()) } }