use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N3_Gcd__0 () { type A = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = Z0 ; type P3 = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type N3Gcd_0 = < < A as Gcd < B > > :: Output as Same < P3 > > :: Output ; assert_eq ! (< N3Gcd_0 as Integer >:: to_i64 () , < P3 as Integer >:: to_i64 ()) ; }
}