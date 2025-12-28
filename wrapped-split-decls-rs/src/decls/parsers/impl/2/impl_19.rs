use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Parse for InlineTable { fn parse (input : ParseStream) -> Result < Self > { let content ; braced ! (content in input) ; Ok (InlineTable { items : Punctuated :: parse_terminated (& content) ? , }) } }