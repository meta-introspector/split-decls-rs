use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_5_BitXor_1 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type B = UInt < UTerm , B1 > ; type U4 = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U5BitXorU1 = < < A as BitXor < B > > :: Output as Same < U4 > > :: Output ; assert_eq ! (< U5BitXorU1 as Unsigned >:: to_u64 () , < U4 as Unsigned >:: to_u64 ()) ; }