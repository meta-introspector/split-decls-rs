use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn boxed () { let boxed : Box < [u8] > = vec ! [1 , 1 , 2 , 3 , 5 , 8 , 13 , 21] . into_boxed_slice () ; let slice = slice_owned (boxed , Deref :: deref) ; assert_eq ! (&* slice , [1 , 1 , 2 , 3 , 5 , 8 , 13 , 21]) ; }