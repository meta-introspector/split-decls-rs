use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N1_Rem_N3 () { type A = NInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type N1 = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type N1RemN3 = < < A as Rem < B > > :: Output as Same < N1 > > :: Output ; assert_eq ! (< N1RemN3 as Integer >:: to_i64 () , < N1 as Integer >:: to_i64 ()) ; }