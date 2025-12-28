use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test__0_PartialDiv_N1 () { type A = Z0 ; type B = NInt < UInt < UTerm , B1 > > ; type _0 = Z0 ; # [allow (non_camel_case_types)] type _0PartialDivN1 = < < A as PartialDiv < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< _0PartialDivN1 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }
}