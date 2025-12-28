use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test__0_Add_P4 () { type A = Z0 ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type P4 = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type _0AddP4 = < < A as Add < B > > :: Output as Same < P4 > > :: Output ; assert_eq ! (< _0AddP4 as Integer >:: to_i64 () , < P4 as Integer >:: to_i64 ()) ; }
}