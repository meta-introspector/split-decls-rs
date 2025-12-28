use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_0_Min_1 () { type A = UTerm ; type B = UInt < UTerm , B1 > ; type U0 = UTerm ; # [allow (non_camel_case_types)] type U0MinU1 = < < A as Min < B > > :: Output as Same < U0 > > :: Output ; assert_eq ! (< U0MinU1 as Unsigned >:: to_u64 () , < U0 as Unsigned >:: to_u64 ()) ; }