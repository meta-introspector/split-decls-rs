use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P3_Min_P5 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P3 = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type P3MinP5 = < < A as Min < B > > :: Output as Same < P3 > > :: Output ; assert_eq ! (< P3MinP5 as Integer >:: to_i64 () , < P3 as Integer >:: to_i64 ()) ; }
}