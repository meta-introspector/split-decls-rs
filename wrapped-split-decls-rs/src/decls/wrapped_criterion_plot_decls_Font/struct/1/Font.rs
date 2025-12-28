use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A font name"] pub struct Font (Cow < 'static , str >) ;
}