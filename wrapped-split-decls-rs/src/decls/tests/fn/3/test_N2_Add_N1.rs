use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N2_Add_N1 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = NInt < UInt < UTerm , B1 > > ; type N3 = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type N2AddN1 = < < A as Add < B > > :: Output as Same < N3 > > :: Output ; assert_eq ! (< N2AddN1 as Integer >:: to_i64 () , < N3 as Integer >:: to_i64 ()) ; }