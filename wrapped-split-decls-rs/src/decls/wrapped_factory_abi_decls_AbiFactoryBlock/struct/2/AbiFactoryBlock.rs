use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [repr (C)] pub struct AbiFactoryBlock { pub block_ptr : * mut c_void , pub get_name : FactoryBlockGetName , pub get_cost : FactoryBlockGetCost , pub execute : FactoryBlockExecute , }