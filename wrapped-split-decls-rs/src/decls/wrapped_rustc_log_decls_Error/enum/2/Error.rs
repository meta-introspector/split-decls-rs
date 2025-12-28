use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug)] pub enum Error { InvalidColorValue (String) , NonUnicodeColorValue , InvalidWraptree (String) , AlreadyInit (SetGlobalDefaultError) , }
}