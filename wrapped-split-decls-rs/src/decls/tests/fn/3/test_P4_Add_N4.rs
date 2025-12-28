use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P4_Add_N4 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type _0 = Z0 ; # [allow (non_camel_case_types)] type P4AddN4 = < < A as Add < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< P4AddN4 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }
}