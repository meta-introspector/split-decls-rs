use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_1_BitXor_0 () { type A = UInt < UTerm , B1 > ; type B = UTerm ; type U1 = UInt < UTerm , B1 > ; # [allow (non_camel_case_types)] type U1BitXorU0 = < < A as BitXor < B > > :: Output as Same < U1 > > :: Output ; assert_eq ! (< U1BitXorU0 as Unsigned >:: to_u64 () , < U1 as Unsigned >:: to_u64 ()) ; }