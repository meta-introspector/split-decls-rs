use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Output file path"] pub struct Output (Cow < 'static , Path >) ;
}