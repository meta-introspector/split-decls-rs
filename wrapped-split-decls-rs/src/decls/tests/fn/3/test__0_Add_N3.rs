use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test__0_Add_N3 () { type A = Z0 ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type N3 = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type _0AddN3 = < < A as Add < B > > :: Output as Same < N3 > > :: Output ; assert_eq ! (< _0AddN3 as Integer >:: to_i64 () , < N3 as Integer >:: to_i64 ()) ; }