use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_3_Pow_3 () { type A = UInt < UInt < UTerm , B1 > , B1 > ; type B = UInt < UInt < UTerm , B1 > , B1 > ; type U27 = UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > , B1 > , B1 > ; # [allow (non_camel_case_types)] type U3PowU3 = < < A as Pow < B > > :: Output as Same < U27 > > :: Output ; assert_eq ! (< U3PowU3 as Unsigned >:: to_u64 () , < U27 as Unsigned >:: to_u64 ()) ; }