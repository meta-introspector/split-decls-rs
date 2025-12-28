use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_0_Gcd_1 () { type A = UTerm ; type B = UInt < UTerm , B1 > ; type U1 = UInt < UTerm , B1 > ; # [allow (non_camel_case_types)] type U0GcdU1 = < < A as Gcd < B > > :: Output as Same < U1 > > :: Output ; assert_eq ! (< U0GcdU1 as Unsigned >:: to_u64 () , < U1 as Unsigned >:: to_u64 ()) ; }