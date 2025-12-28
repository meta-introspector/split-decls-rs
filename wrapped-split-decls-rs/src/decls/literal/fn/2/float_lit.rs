use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn float_lit (symbol : Symbol , suffix : Option < Symbol >) -> Result < LitKind , LitError > { debug ! ("float_lit: {:?}, {:?}" , symbol , suffix) ; filtered_float_lit (strip_underscores (symbol) , suffix , 10) }
}