use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test__0_Add_P2 () { type A = Z0 ; type B = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type P2 = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type _0AddP2 = < < A as Add < B > > :: Output as Same < P2 > > :: Output ; assert_eq ! (< _0AddP2 as Integer >:: to_i64 () , < P2 as Integer >:: to_i64 ()) ; }
}