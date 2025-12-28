use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P2_Div_N1 () { type A = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = NInt < UInt < UTerm , B1 > > ; type N2 = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type P2DivN1 = < < A as Div < B > > :: Output as Same < N2 > > :: Output ; assert_eq ! (< P2DivN1 as Integer >:: to_i64 () , < N2 as Integer >:: to_i64 ()) ; }
}