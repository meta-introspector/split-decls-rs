use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P2_Div_N2 () { type A = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type N1 = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type P2DivN2 = < < A as Div < B > > :: Output as Same < N1 > > :: Output ; assert_eq ! (< P2DivN2 as Integer >:: to_i64 () , < N1 as Integer >:: to_i64 ()) ; }
}