use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Serialize , Deserialize)] pub struct SyscallInterceptor { pub syscall_mappings : HashMap < String , SyscallWrapper > , pub mock_mode : bool , pub dao_governance : bool , pub type_safety_level : TypeSafetyLevel , }