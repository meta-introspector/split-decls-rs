use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test__0_Min_N1 () { type A = Z0 ; type B = NInt < UInt < UTerm , B1 > > ; type N1 = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type _0MinN1 = < < A as Min < B > > :: Output as Same < N1 > > :: Output ; assert_eq ! (< _0MinN1 as Integer >:: to_i64 () , < N1 as Integer >:: to_i64 ()) ; }
}