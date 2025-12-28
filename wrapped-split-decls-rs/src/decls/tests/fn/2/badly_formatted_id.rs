use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn badly_formatted_id () { let id2 = Id :: new ("Weird { struct : ure } !!!") ; match id2 { Ok (_) => panic ! ("graphviz id suddenly allows spaces, brackets and stuff") , Err (..) => { } } }
}