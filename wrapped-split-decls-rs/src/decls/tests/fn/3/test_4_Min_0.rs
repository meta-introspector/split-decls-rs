use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_4_Min_0 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type B = UTerm ; type U0 = UTerm ; # [allow (non_camel_case_types)] type U4MinU0 = < < A as Min < B > > :: Output as Same < U0 > > :: Output ; assert_eq ! (< U4MinU0 as Unsigned >:: to_u64 () , < U0 as Unsigned >:: to_u64 ()) ; }