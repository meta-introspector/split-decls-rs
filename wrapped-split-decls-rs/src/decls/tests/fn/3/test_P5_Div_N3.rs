use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P5_Div_N3 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type N1 = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type P5DivN3 = < < A as Div < B > > :: Output as Same < N1 > > :: Output ; assert_eq ! (< P5DivN3 as Integer >:: to_i64 () , < N1 as Integer >:: to_i64 ()) ; }
}