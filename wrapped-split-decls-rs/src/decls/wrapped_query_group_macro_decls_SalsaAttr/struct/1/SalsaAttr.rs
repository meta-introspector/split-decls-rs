use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct SalsaAttr { name : String , tts : TokenStream , span : Span , }
}