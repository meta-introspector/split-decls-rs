use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl FromIterator < TokenTree > for TokenStream { fn from_iter < I : IntoIterator < Item = TokenTree > > (iter : I) -> Self { TokenStream :: new (iter . into_iter () . collect :: < Vec < TokenTree > > ()) } }