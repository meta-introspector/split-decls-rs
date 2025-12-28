use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P2_Sub_P2 () { type A = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type _0 = Z0 ; # [allow (non_camel_case_types)] type P2SubP2 = < < A as Sub < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< P2SubP2 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }
}