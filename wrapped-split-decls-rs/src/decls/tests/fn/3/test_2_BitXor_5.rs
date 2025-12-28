use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_2_BitXor_5 () { type A = UInt < UInt < UTerm , B1 > , B0 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type U7 = UInt < UInt < UInt < UTerm , B1 > , B1 > , B1 > ; # [allow (non_camel_case_types)] type U2BitXorU5 = < < A as BitXor < B > > :: Output as Same < U7 > > :: Output ; assert_eq ! (< U2BitXorU5 as Unsigned >:: to_u64 () , < U7 as Unsigned >:: to_u64 ()) ; }