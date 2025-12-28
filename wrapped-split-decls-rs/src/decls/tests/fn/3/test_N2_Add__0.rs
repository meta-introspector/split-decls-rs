use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N2_Add__0 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = Z0 ; type N2 = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N2Add_0 = < < A as Add < B > > :: Output as Same < N2 > > :: Output ; assert_eq ! (< N2Add_0 as Integer >:: to_i64 () , < N2 as Integer >:: to_i64 ()) ; }