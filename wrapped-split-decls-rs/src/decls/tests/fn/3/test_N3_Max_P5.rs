use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N3_Max_P5 () { type A = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P5 = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type N3MaxP5 = < < A as Max < B > > :: Output as Same < P5 > > :: Output ; assert_eq ! (< N3MaxP5 as Integer >:: to_i64 () , < P5 as Integer >:: to_i64 ()) ; }
}