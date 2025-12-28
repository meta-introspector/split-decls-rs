use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn check_dup_symbol () { let input = quote ! { Keywords { } Symbols { splat , splat , } } ; test_symbols_macro (input , & ["Symbol `splat` is duplicated" , "location of previous definition"]) ; }