use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Parse for RootInput { fn parse (input : ParseStream) -> Result < Self > { Ok (RootInput { items : Punctuated :: parse_terminated (input) ? , }) } }