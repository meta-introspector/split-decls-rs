use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone)] pub struct TokenStreamIter < 't > { stream : & 't TokenStream , index : usize , }
}