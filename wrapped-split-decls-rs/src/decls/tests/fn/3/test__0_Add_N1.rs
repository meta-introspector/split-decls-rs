use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test__0_Add_N1 () { type A = Z0 ; type B = NInt < UInt < UTerm , B1 > > ; type N1 = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type _0AddN1 = < < A as Add < B > > :: Output as Same < N1 > > :: Output ; assert_eq ! (< _0AddN1 as Integer >:: to_i64 () , < N1 as Integer >:: to_i64 ()) ; }
}