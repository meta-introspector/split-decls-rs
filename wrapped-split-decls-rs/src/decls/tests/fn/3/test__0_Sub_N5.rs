use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test__0_Sub_N5 () { type A = Z0 ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P5 = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type _0SubN5 = < < A as Sub < B > > :: Output as Same < P5 > > :: Output ; assert_eq ! (< _0SubN5 as Integer >:: to_i64 () , < P5 as Integer >:: to_i64 ()) ; }
}