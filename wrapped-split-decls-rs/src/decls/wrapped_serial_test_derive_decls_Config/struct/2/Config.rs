use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Default , Debug)] struct Config { names : Vec < String > , path : QuoteOption < String > , crate_ident : Vec < TokenTree > , }