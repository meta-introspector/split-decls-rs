use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N1_Rem_P5 () { type A = NInt < UInt < UTerm , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N1 = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type N1RemP5 = < < A as Rem < B > > :: Output as Same < N1 > > :: Output ; assert_eq ! (< N1RemP5 as Integer >:: to_i64 () , < N1 as Integer >:: to_i64 ()) ; }