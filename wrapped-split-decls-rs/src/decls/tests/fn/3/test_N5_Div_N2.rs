use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N5_Div_N2 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type P2 = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N5DivN2 = < < A as Div < B > > :: Output as Same < P2 > > :: Output ; assert_eq ! (< N5DivN2 as Integer >:: to_i64 () , < P2 as Integer >:: to_i64 ()) ; }
}