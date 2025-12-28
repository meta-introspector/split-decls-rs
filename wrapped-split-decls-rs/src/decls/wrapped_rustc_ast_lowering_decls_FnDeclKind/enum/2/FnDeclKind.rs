use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Copy , Clone , Debug , PartialEq , Eq)] enum FnDeclKind { Fn , Inherent , ExternFn , Closure , Pointer , Trait , Impl , }
}