use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: panic_b_still_executes");
# [test] # [cfg_attr (not (panic = "unwind") , ignore)] fn panic_b_still_executes () { let mut x = false ; match unwind :: halt_unwinding (| | join (| | panic ! ("Hello, world!") , | | x = true)) { Ok (_) => panic ! ("failed to propagate panic from closure A,") , Err (_) => assert ! (x , "closure b failed to execute") , } }
}