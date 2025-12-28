use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P5_Rem_N3 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type P2 = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type P5RemN3 = < < A as Rem < B > > :: Output as Same < P2 > > :: Output ; assert_eq ! (< P5RemN3 as Integer >:: to_i64 () , < P2 as Integer >:: to_i64 ()) ; }