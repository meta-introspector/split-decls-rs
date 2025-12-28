use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn mixed_lifetime_scope () { fn increment < 'slice , 'counter > (counters : & 'slice [& 'counter AtomicUsize]) { scope (move | s : & Scope < 'counter > | { for & c in counters { s . spawn (move | _ | { c . fetch_add (1 , Ordering :: Relaxed) ; }) ; } }) ; } let counter = AtomicUsize :: new (0) ; increment (& [& counter ; 100]) ; assert_eq ! (counter . into_inner () , 100) ; }
}