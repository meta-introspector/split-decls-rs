use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_0_Shl_0 () { type A = UTerm ; type B = UTerm ; type U0 = UTerm ; # [allow (non_camel_case_types)] type U0ShlU0 = < < A as Shl < B > > :: Output as Same < U0 > > :: Output ; assert_eq ! (< U0ShlU0 as Unsigned >:: to_u64 () , < U0 as Unsigned >:: to_u64 ()) ; }
}