use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P3_Div_N1 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = NInt < UInt < UTerm , B1 > > ; type N3 = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type P3DivN1 = < < A as Div < B > > :: Output as Same < N3 > > :: Output ; assert_eq ! (< P3DivN1 as Integer >:: to_i64 () , < N3 as Integer >:: to_i64 ()) ; }
}