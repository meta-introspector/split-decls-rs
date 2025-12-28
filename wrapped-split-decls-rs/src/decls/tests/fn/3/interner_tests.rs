use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn interner_tests () { let i = Interner :: prefill (& [] , & []) ; assert_eq ! (i . intern_str ("dog") , Symbol :: new (0)) ; assert_eq ! (i . intern_byte_str (b"dog") , ByteSymbol :: new (0)) ; assert_eq ! (i . intern_byte_str (b"cat") , ByteSymbol :: new (1)) ; assert_eq ! (i . intern_str ("cat") , Symbol :: new (1)) ; assert_eq ! (i . intern_str ("dog") , Symbol :: new (0)) ; }
}