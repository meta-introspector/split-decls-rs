use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct ExpressionVisitor { expressions : HashMap < String , Vec < String > > , current_file : String , }
}