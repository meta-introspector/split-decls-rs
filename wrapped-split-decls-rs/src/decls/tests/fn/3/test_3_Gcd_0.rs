use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_3_Gcd_0 () { type A = UInt < UInt < UTerm , B1 > , B1 > ; type B = UTerm ; type U3 = UInt < UInt < UTerm , B1 > , B1 > ; # [allow (non_camel_case_types)] type U3GcdU0 = < < A as Gcd < B > > :: Output as Same < U3 > > :: Output ; assert_eq ! (< U3GcdU0 as Unsigned >:: to_u64 () , < U3 as Unsigned >:: to_u64 ()) ; }
}