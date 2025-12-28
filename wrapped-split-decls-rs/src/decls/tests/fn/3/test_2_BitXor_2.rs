use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_2_BitXor_2 () { type A = UInt < UInt < UTerm , B1 > , B0 > ; type B = UInt < UInt < UTerm , B1 > , B0 > ; type U0 = UTerm ; # [allow (non_camel_case_types)] type U2BitXorU2 = < < A as BitXor < B > > :: Output as Same < U0 > > :: Output ; assert_eq ! (< U2BitXorU2 as Unsigned >:: to_u64 () , < U0 as Unsigned >:: to_u64 ()) ; }