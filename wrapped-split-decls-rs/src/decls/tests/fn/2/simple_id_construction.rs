use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn simple_id_construction () { let id1 = Id :: new ("hello") ; match id1 { Ok (_) => { } Err (..) => panic ! ("'hello' is not a valid value for id anymore") , } }
}