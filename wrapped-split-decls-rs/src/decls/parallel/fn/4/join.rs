use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [inline] pub fn join < A , B , RA : DynSend , RB : DynSend > (oper_a : A , oper_b : B) -> (RA , RB) where A : FnOnce () -> RA + DynSend , B : FnOnce () -> RB + DynSend , { if mode :: is_dyn_thread_safe () { let oper_a = FromDyn :: from (oper_a) ; let oper_b = FromDyn :: from (oper_b) ; let (a , b) = parallel_guard (| guard | { rustc_thread_pool :: join (move | | guard . run (move | | FromDyn :: from (oper_a . into_inner () ())) , move | | guard . run (move | | FromDyn :: from (oper_b . into_inner () ())) ,) }) ; (a . unwrap () . into_inner () , b . unwrap () . into_inner ()) } else { serial_join (oper_a , oper_b) } }
}