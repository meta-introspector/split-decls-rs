use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_5_Gcd_2 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type B = UInt < UInt < UTerm , B1 > , B0 > ; type U1 = UInt < UTerm , B1 > ; # [allow (non_camel_case_types)] type U5GcdU2 = < < A as Gcd < B > > :: Output as Same < U1 > > :: Output ; assert_eq ! (< U5GcdU2 as Unsigned >:: to_u64 () , < U1 as Unsigned >:: to_u64 ()) ; }