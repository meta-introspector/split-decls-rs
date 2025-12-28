use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P2_Div_P4 () { type A = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type _0 = Z0 ; # [allow (non_camel_case_types)] type P2DivP4 = < < A as Div < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< P2DivP4 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }
}