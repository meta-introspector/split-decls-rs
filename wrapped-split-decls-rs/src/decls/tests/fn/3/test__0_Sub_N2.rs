use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test__0_Sub_N2 () { type A = Z0 ; type B = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type P2 = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type _0SubN2 = < < A as Sub < B > > :: Output as Same < P2 > > :: Output ; assert_eq ! (< _0SubN2 as Integer >:: to_i64 () , < P2 as Integer >:: to_i64 ()) ; }
}