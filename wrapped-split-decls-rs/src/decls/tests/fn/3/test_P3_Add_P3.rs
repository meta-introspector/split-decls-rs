use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P3_Add_P3 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type P6 = PInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > > ; # [allow (non_camel_case_types)] type P3AddP3 = < < A as Add < B > > :: Output as Same < P6 > > :: Output ; assert_eq ! (< P3AddP3 as Integer >:: to_i64 () , < P6 as Integer >:: to_i64 ()) ; }
}