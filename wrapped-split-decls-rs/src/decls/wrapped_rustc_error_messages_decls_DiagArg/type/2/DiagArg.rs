use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Simplified version of `FluentArg` that can implement `Encodable` and `Decodable`. Collection of"] # [doc = " `DiagArg` are converted to `FluentArgs` (consuming the collection) at the start of diagnostic"] # [doc = " emission."] pub type DiagArg < 'iter > = (& 'iter DiagArgName , & 'iter DiagArgValue) ;