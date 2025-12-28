use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn slice_owned_the_slice () { let slice = slice_owned (vec ! [1 , 2 , 3 , 4 , 5 , 6] , Vec :: as_slice) ; let slice = slice_owned (slice , | s | & s [1 ..] [.. 4]) ; let slice = slice_owned (slice , | s | s) ; let slice = slice_owned (slice , | s | & s [1 ..]) ; assert_eq ! (&* slice , & [1 , 2 , 3 , 4 , 5 , 6] [1 ..] [.. 4] [1 ..]) ; }