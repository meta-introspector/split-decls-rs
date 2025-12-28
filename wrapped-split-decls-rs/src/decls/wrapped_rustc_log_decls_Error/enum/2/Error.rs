use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug)] pub enum Error { InvalidColorValue (String) , NonUnicodeColorValue , InvalidWraptree (String) , AlreadyInit (SetGlobalDefaultError) , }