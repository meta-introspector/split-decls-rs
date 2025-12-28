use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Copy , Clone , Debug , PartialEq , Eq)] enum FnDeclKind { Fn , Inherent , ExternFn , Closure , Pointer , Trait , Impl , }