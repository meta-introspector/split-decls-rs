use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N5_Div_P2 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type N2 = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N5DivP2 = < < A as Div < B > > :: Output as Same < N2 > > :: Output ; assert_eq ! (< N5DivP2 as Integer >:: to_i64 () , < N2 as Integer >:: to_i64 ()) ; }
}