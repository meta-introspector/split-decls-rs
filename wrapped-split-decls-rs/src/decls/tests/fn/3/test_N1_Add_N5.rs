use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N1_Add_N5 () { type A = NInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N6 = NInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N1AddN5 = < < A as Add < B > > :: Output as Same < N6 > > :: Output ; assert_eq ! (< N1AddN5 as Integer >:: to_i64 () , < N6 as Integer >:: to_i64 ()) ; }