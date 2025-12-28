use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , Copy)] pub struct SubtreeView < 'a , S > (& 'a [TokenTree < S >]) ;
}