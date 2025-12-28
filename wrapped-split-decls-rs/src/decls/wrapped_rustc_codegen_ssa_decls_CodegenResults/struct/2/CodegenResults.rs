use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Encodable , Decodable)] pub struct CodegenResults { pub modules : Vec < CompiledModule > , pub allocator_module : Option < CompiledModule > , pub crate_info : CrateInfo , }