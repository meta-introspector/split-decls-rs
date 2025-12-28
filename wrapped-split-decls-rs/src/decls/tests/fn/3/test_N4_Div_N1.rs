use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N4_Div_N1 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = NInt < UInt < UTerm , B1 > > ; type P4 = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type N4DivN1 = < < A as Div < B > > :: Output as Same < P4 > > :: Output ; assert_eq ! (< N4DivN1 as Integer >:: to_i64 () , < P4 as Integer >:: to_i64 ()) ; }
}