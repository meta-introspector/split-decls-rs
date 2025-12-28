use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_2_Gcd_0 () { type A = UInt < UInt < UTerm , B1 > , B0 > ; type B = UTerm ; type U2 = UInt < UInt < UTerm , B1 > , B0 > ; # [allow (non_camel_case_types)] type U2GcdU0 = < < A as Gcd < B > > :: Output as Same < U2 > > :: Output ; assert_eq ! (< U2GcdU0 as Unsigned >:: to_u64 () , < U2 as Unsigned >:: to_u64 ()) ; }
}