use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Copy)] pub struct SubtreeView < 'a , S > (& 'a [TokenTree < S >]) ;