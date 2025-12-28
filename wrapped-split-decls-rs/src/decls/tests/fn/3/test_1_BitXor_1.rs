use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_1_BitXor_1 () { type A = UInt < UTerm , B1 > ; type B = UInt < UTerm , B1 > ; type U0 = UTerm ; # [allow (non_camel_case_types)] type U1BitXorU1 = < < A as BitXor < B > > :: Output as Same < U0 > > :: Output ; assert_eq ! (< U1BitXorU1 as Unsigned >:: to_u64 () , < U0 as Unsigned >:: to_u64 ()) ; }