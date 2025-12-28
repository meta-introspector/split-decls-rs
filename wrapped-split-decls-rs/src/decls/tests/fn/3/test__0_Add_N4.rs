use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test__0_Add_N4 () { type A = Z0 ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type N4 = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type _0AddN4 = < < A as Add < B > > :: Output as Same < N4 > > :: Output ; assert_eq ! (< _0AddN4 as Integer >:: to_i64 () , < N4 as Integer >:: to_i64 ()) ; }
}