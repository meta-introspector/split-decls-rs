use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl FnHeader { # [doc = " Does this function header have any qualifiers or is it empty?"] pub fn has_qualifiers (& self) -> bool { let Self { safety , coroutine_kind , constness , ext } = self ; matches ! (safety , Safety :: Unsafe (_)) || coroutine_kind . is_some () || matches ! (constness , Const :: Yes (_)) || ! matches ! (ext , Extern :: None) } }