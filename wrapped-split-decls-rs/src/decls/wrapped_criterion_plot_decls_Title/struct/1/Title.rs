use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Figure title"] pub struct Title (Cow < 'static , str >) ;
}