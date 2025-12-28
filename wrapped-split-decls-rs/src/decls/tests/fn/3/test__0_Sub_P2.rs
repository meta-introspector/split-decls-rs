use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test__0_Sub_P2 () { type A = Z0 ; type B = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type N2 = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type _0SubP2 = < < A as Sub < B > > :: Output as Same < N2 > > :: Output ; assert_eq ! (< _0SubP2 as Integer >:: to_i64 () , < N2 as Integer >:: to_i64 ()) ; }
}