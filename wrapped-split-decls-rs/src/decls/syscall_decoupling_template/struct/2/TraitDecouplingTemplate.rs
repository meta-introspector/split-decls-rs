use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Serialize , Deserialize)] pub struct TraitDecouplingTemplate { pub syscall_traits : Vec < SyscallTrait > , pub wrapper_macros : Vec < WrapperMacro > , pub implementation_adapters : Vec < ImplAdapter > , }
}