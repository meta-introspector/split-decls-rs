use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N1_Div_P4 () { type A = NInt < UInt < UTerm , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type _0 = Z0 ; # [allow (non_camel_case_types)] type N1DivP4 = < < A as Div < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< N1DivP4 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }
}