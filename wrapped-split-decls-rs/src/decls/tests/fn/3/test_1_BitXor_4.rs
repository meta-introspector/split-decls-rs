use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_1_BitXor_4 () { type A = UInt < UTerm , B1 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type U5 = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; # [allow (non_camel_case_types)] type U1BitXorU4 = < < A as BitXor < B > > :: Output as Same < U5 > > :: Output ; assert_eq ! (< U1BitXorU4 as Unsigned >:: to_u64 () , < U5 as Unsigned >:: to_u64 ()) ; }