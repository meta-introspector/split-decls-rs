use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " If this part of two syntax contexts is equal, then the whole syntax contexts should be equal."] # [doc = " The other fields are only for caching."] pub type SyntaxContextKey = (SyntaxContext , ExpnId , Transparency) ;