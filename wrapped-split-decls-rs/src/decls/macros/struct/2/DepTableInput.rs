use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct DepTableInput { name : LitStr , _comma_token : Token ! [,] , table_content : proc_macro2 :: TokenStream , }
}