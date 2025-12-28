use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test__0_Sub_P4 () { type A = Z0 ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type N4 = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type _0SubP4 = < < A as Sub < B > > :: Output as Same < N4 > > :: Output ; assert_eq ! (< _0SubP4 as Integer >:: to_i64 () , < N4 as Integer >:: to_i64 ()) ; }
}