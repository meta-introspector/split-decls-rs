use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test__0_Min_N2 () { type A = Z0 ; type B = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type N2 = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type _0MinN2 = < < A as Min < B > > :: Output as Same < N2 > > :: Output ; assert_eq ! (< _0MinN2 as Integer >:: to_i64 () , < N2 as Integer >:: to_i64 ()) ; }
}