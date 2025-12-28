use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_0_BitXor_5 () { type A = UTerm ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type U5 = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; # [allow (non_camel_case_types)] type U0BitXorU5 = < < A as BitXor < B > > :: Output as Same < U5 > > :: Output ; assert_eq ! (< U0BitXorU5 as Unsigned >:: to_u64 () , < U5 as Unsigned >:: to_u64 ()) ; }