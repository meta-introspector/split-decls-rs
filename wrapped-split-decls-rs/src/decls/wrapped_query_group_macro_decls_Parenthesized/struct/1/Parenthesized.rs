use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Parenthesis helper"] pub (crate) struct Parenthesized < T > (pub (crate) T) ;