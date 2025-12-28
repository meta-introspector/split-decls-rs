use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_1_BitXor_5 () { type A = UInt < UTerm , B1 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type U4 = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U1BitXorU5 = < < A as BitXor < B > > :: Output as Same < U4 > > :: Output ; assert_eq ! (< U1BitXorU5 as Unsigned >:: to_u64 () , < U4 as Unsigned >:: to_u64 ()) ; }