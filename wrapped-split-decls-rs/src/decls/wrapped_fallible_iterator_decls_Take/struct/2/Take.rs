use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " An iterator which yields a limited number of elements from the underlying"] # [doc = " iterator."] # [derive (Clone , Debug)] pub struct Take < I > { it : I , remaining : usize , }