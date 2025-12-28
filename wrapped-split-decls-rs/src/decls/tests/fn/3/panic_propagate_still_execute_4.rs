use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [cfg_attr (not (panic = "unwind") , ignore)] fn panic_propagate_still_execute_4 () { let mut x = false ; let result = unwind :: halt_unwinding (| | { scope (| s | { s . spawn (| _ | panic ! ("Hello, world!")) ; x = true ; }) ; }) ; match result { Ok (_) => panic ! ("failed to propagate panic") , Err (_) => assert ! (x , "panic in spawn tainted scope") , } }