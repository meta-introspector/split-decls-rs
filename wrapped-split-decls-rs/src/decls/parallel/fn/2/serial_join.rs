use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn serial_join < A , B , RA , RB > (oper_a : A , oper_b : B) -> (RA , RB) where A : FnOnce () -> RA , B : FnOnce () -> RB , { let (a , b) = parallel_guard (| guard | { let a = guard . run (oper_a) ; let b = guard . run (oper_b) ; (a , b) }) ; (a . unwrap () , b . unwrap ()) }