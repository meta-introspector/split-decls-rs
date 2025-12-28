use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Options to control the demangling process."] # [derive (Clone , Copy , Debug , Default)] # [repr (C)] pub struct DemangleOptions { no_params : bool , no_return_type : bool , hide_expression_literal_types : bool , recursion_limit : Option < NonZeroU32 > , }