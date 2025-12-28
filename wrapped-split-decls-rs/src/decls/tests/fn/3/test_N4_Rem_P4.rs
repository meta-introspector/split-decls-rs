use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N4_Rem_P4 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type _0 = Z0 ; # [allow (non_camel_case_types)] type N4RemP4 = < < A as Rem < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< N4RemP4 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }
}