use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct WriteInput { dst : Expr , rest : TokenStream , }
}