use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_4_BitXor_0 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type B = UTerm ; type U4 = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U4BitXorU0 = < < A as BitXor < B > > :: Output as Same < U4 > > :: Output ; assert_eq ! (< U4BitXorU0 as Unsigned >:: to_u64 () , < U4 as Unsigned >:: to_u64 ()) ; }