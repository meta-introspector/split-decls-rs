use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < Ident > for TokenTree { fn from (g : Ident) -> Self { TokenTree :: Ident (g) } }