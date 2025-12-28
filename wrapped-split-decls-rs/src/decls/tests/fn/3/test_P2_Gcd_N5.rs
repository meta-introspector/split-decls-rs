use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P2_Gcd_N5 () { type A = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P1 = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type P2GcdN5 = < < A as Gcd < B > > :: Output as Same < P1 > > :: Output ; assert_eq ! (< P2GcdN5 as Integer >:: to_i64 () , < P1 as Integer >:: to_i64 ()) ; }
}