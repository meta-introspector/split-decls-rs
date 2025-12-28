use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N2_Div_N1 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = NInt < UInt < UTerm , B1 > > ; type P2 = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N2DivN1 = < < A as Div < B > > :: Output as Same < P2 > > :: Output ; assert_eq ! (< N2DivN1 as Integer >:: to_i64 () , < P2 as Integer >:: to_i64 ()) ; }
}