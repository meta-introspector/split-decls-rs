use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: scope_two");
# [test] fn scope_two () { let counter = & AtomicUsize :: new (0) ; scope (| s | { s . spawn (move | _ | { counter . fetch_add (1 , Ordering :: SeqCst) ; }) ; s . spawn (move | _ | { counter . fetch_add (10 , Ordering :: SeqCst) ; }) ; }) ; let v = counter . load (Ordering :: SeqCst) ; assert_eq ! (v , 11) ; }
}