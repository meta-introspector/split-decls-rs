use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn check_dup_symbol_and_keyword () { let input = quote ! { Keywords { Splat : "splat" , } Symbols { splat , } } ; test_symbols_macro (input , & ["Symbol `splat` is duplicated" , "location of previous definition"]) ; }