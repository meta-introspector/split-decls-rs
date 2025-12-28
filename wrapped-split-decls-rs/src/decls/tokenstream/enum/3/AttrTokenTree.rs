use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Like `TokenTree`, but for `AttrTokenStream`."] # [derive (Clone , Debug , Encodable , Decodable)] pub enum AttrTokenTree { Token (Token , Spacing) , Delimited (DelimSpan , DelimSpacing , Delimiter , AttrTokenStream) , # [doc = " Stores the attributes for an attribute target,"] # [doc = " along with the tokens for that attribute target."] # [doc = " See `AttrsTarget` for more information"] AttrsTarget (AttrsTarget) , }