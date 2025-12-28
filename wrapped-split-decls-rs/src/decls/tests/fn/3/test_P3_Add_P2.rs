use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P3_Add_P2 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type P5 = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type P3AddP2 = < < A as Add < B > > :: Output as Same < P5 > > :: Output ; assert_eq ! (< P3AddP2 as Integer >:: to_i64 () , < P5 as Integer >:: to_i64 ()) ; }
}