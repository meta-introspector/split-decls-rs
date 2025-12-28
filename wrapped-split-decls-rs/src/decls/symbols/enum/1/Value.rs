use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
enum Value { SameAsName , String (LitStr) , Env (LitStr , Macro) , Unsupported (Expr) , }
}