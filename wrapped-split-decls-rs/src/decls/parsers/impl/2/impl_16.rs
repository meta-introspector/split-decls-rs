use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Parse for BracketedStringList { fn parse (input : ParseStream) -> Result < Self > { let content ; bracketed ! (content in input) ; Ok (BracketedStringList { list : Punctuated :: parse_terminated (& content) ? , }) } }