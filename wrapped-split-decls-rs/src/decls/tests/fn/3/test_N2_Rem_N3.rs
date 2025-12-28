use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N2_Rem_N3 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type N2 = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N2RemN3 = < < A as Rem < B > > :: Output as Same < N2 > > :: Output ; assert_eq ! (< N2RemN3 as Integer >:: to_i64 () , < N2 as Integer >:: to_i64 ()) ; }