use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N4_Add__0 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = Z0 ; type N4 = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type N4Add_0 = < < A as Add < B > > :: Output as Same < N4 > > :: Output ; assert_eq ! (< N4Add_0 as Integer >:: to_i64 () , < N4 as Integer >:: to_i64 ()) ; }