use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_3_Pow_4 () { type A = UInt < UInt < UTerm , B1 > , B1 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type U81 = UInt < UInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > , B0 > , B0 > , B0 > , B1 > ; # [allow (non_camel_case_types)] type U3PowU4 = < < A as Pow < B > > :: Output as Same < U81 > > :: Output ; assert_eq ! (< U3PowU4 as Unsigned >:: to_u64 () , < U81 as Unsigned >:: to_u64 ()) ; }