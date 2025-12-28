use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl FnDecl { pub fn has_self (& self) -> bool { self . inputs . get (0) . is_some_and (Param :: is_self) } pub fn c_variadic (& self) -> bool { self . inputs . last () . is_some_and (| arg | matches ! (arg . ty . kind , TyKind :: CVarArgs)) } }
}