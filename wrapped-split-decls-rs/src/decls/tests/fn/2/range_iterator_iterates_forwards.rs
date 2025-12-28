use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn range_iterator_iterates_forwards () { let range = MyIdx :: from_u32 (1) .. MyIdx :: from_u32 (4) ; assert_eq ! (range . collect ::< Vec < _ >> () , [MyIdx :: from_u32 (1) , MyIdx :: from_u32 (2) , MyIdx :: from_u32 (3)]) ; }