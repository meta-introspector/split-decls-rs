use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone)] pub struct TokenStreamIter < 't > { stream : & 't TokenStream , index : usize , }