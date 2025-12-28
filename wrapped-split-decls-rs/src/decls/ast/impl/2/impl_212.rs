use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl NormalAttr { pub fn from_ident (ident : Ident) -> Self { Self { item : AttrItem { unsafety : Safety :: Default , path : Path :: from_ident (ident) , args : AttrArgs :: Empty , tokens : None , } , tokens : None , } } }