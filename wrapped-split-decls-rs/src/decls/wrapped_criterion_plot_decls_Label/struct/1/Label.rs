use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Plot label"] pub struct Label (Cow < 'static , str >) ;
}