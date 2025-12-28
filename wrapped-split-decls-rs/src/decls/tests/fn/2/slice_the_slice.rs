use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn slice_the_slice () { let slice = slice_owned (vec ! [1 , 2 , 3 , 4 , 5 , 6] , Vec :: as_slice) . slice (| s | & s [1 ..] [.. 4]) . slice (| s | s) . slice (| s | & s [1 ..]) ; assert_eq ! (&* slice , & [1 , 2 , 3 , 4 , 5 , 6] [1 ..] [.. 4] [1 ..]) ; }