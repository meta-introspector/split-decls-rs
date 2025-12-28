use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P3_Rem_N2 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type P1 = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type P3RemN2 = < < A as Rem < B > > :: Output as Same < P1 > > :: Output ; assert_eq ! (< P3RemN2 as Integer >:: to_i64 () , < P1 as Integer >:: to_i64 ()) ; }