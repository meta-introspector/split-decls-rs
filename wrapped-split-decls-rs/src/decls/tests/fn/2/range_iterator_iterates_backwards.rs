use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn range_iterator_iterates_backwards () { let range = MyIdx :: from_u32 (1) .. MyIdx :: from_u32 (4) ; assert_eq ! (range . rev () . collect ::< Vec < _ >> () , [MyIdx :: from_u32 (3) , MyIdx :: from_u32 (2) , MyIdx :: from_u32 (1)]) ; }
}