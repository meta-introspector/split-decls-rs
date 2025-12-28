use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Like `Symbol`, but for byte strings. `ByteSymbol` is used less widely, so"] # [doc = " it has fewer operations defined than `Symbol`."] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct ByteSymbol (SymbolIndex) ;