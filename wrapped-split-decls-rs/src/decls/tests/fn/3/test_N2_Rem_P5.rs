use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N2_Rem_P5 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N2 = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N2RemP5 = < < A as Rem < B > > :: Output as Same < N2 > > :: Output ; assert_eq ! (< N2RemP5 as Integer >:: to_i64 () , < N2 as Integer >:: to_i64 ()) ; }