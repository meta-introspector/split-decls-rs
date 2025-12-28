use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_0_Shr_0 () { type A = UTerm ; type B = UTerm ; type U0 = UTerm ; # [allow (non_camel_case_types)] type U0ShrU0 = < < A as Shr < B > > :: Output as Same < U0 > > :: Output ; assert_eq ! (< U0ShrU0 as Unsigned >:: to_u64 () , < U0 as Unsigned >:: to_u64 ()) ; }