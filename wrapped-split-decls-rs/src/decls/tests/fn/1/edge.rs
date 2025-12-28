use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn edge (from : usize , to : usize , label : & 'static str , style : Style) -> Edge { Edge { from , to , label , style } }