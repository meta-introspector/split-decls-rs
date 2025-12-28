use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_2_Pow_0 () { type A = UInt < UInt < UTerm , B1 > , B0 > ; type B = UTerm ; type U1 = UInt < UTerm , B1 > ; # [allow (non_camel_case_types)] type U2PowU0 = < < A as Pow < B > > :: Output as Same < U1 > > :: Output ; assert_eq ! (< U2PowU0 as Unsigned >:: to_u64 () , < U1 as Unsigned >:: to_u64 ()) ; }