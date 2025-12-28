use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test__0_Add_N5 () { type A = Z0 ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N5 = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type _0AddN5 = < < A as Add < B > > :: Output as Same < N5 > > :: Output ; assert_eq ! (< _0AddN5 as Integer >:: to_i64 () , < N5 as Integer >:: to_i64 ()) ; }