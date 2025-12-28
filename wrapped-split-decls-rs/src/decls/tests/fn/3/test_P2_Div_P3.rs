use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P2_Div_P3 () { type A = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type _0 = Z0 ; # [allow (non_camel_case_types)] type P2DivP3 = < < A as Div < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< P2DivP3 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }
}