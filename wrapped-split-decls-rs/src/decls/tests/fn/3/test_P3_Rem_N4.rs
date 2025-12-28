use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P3_Rem_N4 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type P3 = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type P3RemN4 = < < A as Rem < B > > :: Output as Same < P3 > > :: Output ; assert_eq ! (< P3RemN4 as Integer >:: to_i64 () , < P3 as Integer >:: to_i64 ()) ; }