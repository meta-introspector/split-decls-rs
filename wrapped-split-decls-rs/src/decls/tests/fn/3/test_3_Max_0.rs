use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_3_Max_0 () { type A = UInt < UInt < UTerm , B1 > , B1 > ; type B = UTerm ; type U3 = UInt < UInt < UTerm , B1 > , B1 > ; # [allow (non_camel_case_types)] type U3MaxU0 = < < A as Max < B > > :: Output as Same < U3 > > :: Output ; assert_eq ! (< U3MaxU0 as Unsigned >:: to_u64 () , < U3 as Unsigned >:: to_u64 ()) ; }