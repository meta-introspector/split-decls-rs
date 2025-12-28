use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P5_Max__0 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = Z0 ; type P5 = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type P5Max_0 = < < A as Max < B > > :: Output as Same < P5 > > :: Output ; assert_eq ! (< P5Max_0 as Integer >:: to_i64 () , < P5 as Integer >:: to_i64 ()) ; }
}