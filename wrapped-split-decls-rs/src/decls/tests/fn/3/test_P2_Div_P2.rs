use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P2_Div_P2 () { type A = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type P1 = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type P2DivP2 = < < A as Div < B > > :: Output as Same < P1 > > :: Output ; assert_eq ! (< P2DivP2 as Integer >:: to_i64 () , < P1 as Integer >:: to_i64 ()) ; }
}