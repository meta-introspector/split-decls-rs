use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_3_BitXor_1 () { type A = UInt < UInt < UTerm , B1 > , B1 > ; type B = UInt < UTerm , B1 > ; type U2 = UInt < UInt < UTerm , B1 > , B0 > ; # [allow (non_camel_case_types)] type U3BitXorU1 = < < A as BitXor < B > > :: Output as Same < U2 > > :: Output ; assert_eq ! (< U3BitXorU1 as Unsigned >:: to_u64 () , < U2 as Unsigned >:: to_u64 ()) ; }