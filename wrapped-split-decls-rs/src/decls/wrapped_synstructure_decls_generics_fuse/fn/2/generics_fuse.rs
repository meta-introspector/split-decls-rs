use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn generics_fuse (res : & mut Vec < bool > , new : & [bool]) { for (i , & flag) in new . iter () . enumerate () { if i == res . len () { res . push (false) ; } if flag { res [i] = true ; } } }