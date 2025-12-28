use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Contains the sub-token-trees of a \"delimited\" token tree such as `(a b c)`."] # [doc = " The delimiters are not represented explicitly in the `tts` vector."] # [derive (PartialEq , Encodable , Decodable , Debug)] pub struct Delimited { pub delim : Delimiter , # [doc = " FIXME: #67062 has details about why this is sub-optimal."] pub tts : Vec < TokenTree > , }