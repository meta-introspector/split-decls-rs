use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl ToTokens for OracleType { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { let name = match self { OracleType :: FileSystem => "FileSystem" , OracleType :: Network => "Network" , OracleType :: Process => "Process" , OracleType :: Memory => "Memory" , OracleType :: Time => "Time" , OracleType :: Crypto => "Crypto" , OracleType :: Custom (s) => return s . to_tokens (tokens) , } ; tokens . extend (quote :: quote ! { # name }) ; } }