use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn check_dup_keywords () { let input = quote ! { Keywords { Crate : "crate" , Crate : "crate" , } Symbols { } } ; test_symbols_macro (input , & ["Symbol `crate` is duplicated" , "location of previous definition"]) ; }
}