use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct Edge { from : usize , to : usize , label : & 'static str , style : Style , }