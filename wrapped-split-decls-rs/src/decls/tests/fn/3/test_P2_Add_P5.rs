use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P2_Add_P5 () { type A = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P7 = PInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B1 > > ; # [allow (non_camel_case_types)] type P2AddP5 = < < A as Add < B > > :: Output as Same < P7 > > :: Output ; assert_eq ! (< P2AddP5 as Integer >:: to_i64 () , < P7 as Integer >:: to_i64 ()) ; }
}