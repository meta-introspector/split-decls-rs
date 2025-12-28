use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P3_Gcd_P1 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UTerm , B1 > > ; type P1 = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type P3GcdP1 = < < A as Gcd < B > > :: Output as Same < P1 > > :: Output ; assert_eq ! (< P3GcdP1 as Integer >:: to_i64 () , < P1 as Integer >:: to_i64 ()) ; }
}