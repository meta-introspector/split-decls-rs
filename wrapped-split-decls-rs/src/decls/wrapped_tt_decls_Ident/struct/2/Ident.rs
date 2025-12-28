use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Identifier or keyword."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct Ident < S > { pub sym : Symbol , pub span : S , pub is_raw : IdentIsRaw , }