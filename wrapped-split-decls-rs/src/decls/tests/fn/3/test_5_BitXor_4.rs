use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_5_BitXor_4 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type U1 = UInt < UTerm , B1 > ; # [allow (non_camel_case_types)] type U5BitXorU4 = < < A as BitXor < B > > :: Output as Same < U1 > > :: Output ; assert_eq ! (< U5BitXorU4 as Unsigned >:: to_u64 () , < U1 as Unsigned >:: to_u64 ()) ; }