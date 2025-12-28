use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N2_Sub__0 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = Z0 ; type N2 = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N2Sub_0 = < < A as Sub < B > > :: Output as Same < N2 > > :: Output ; assert_eq ! (< N2Sub_0 as Integer >:: to_i64 () , < N2 as Integer >:: to_i64 ()) ; }