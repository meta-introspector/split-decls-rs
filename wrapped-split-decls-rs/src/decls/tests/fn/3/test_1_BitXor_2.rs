use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_1_BitXor_2 () { type A = UInt < UTerm , B1 > ; type B = UInt < UInt < UTerm , B1 > , B0 > ; type U3 = UInt < UInt < UTerm , B1 > , B1 > ; # [allow (non_camel_case_types)] type U1BitXorU2 = < < A as BitXor < B > > :: Output as Same < U3 > > :: Output ; assert_eq ! (< U1BitXorU2 as Unsigned >:: to_u64 () , < U3 as Unsigned >:: to_u64 ()) ; }