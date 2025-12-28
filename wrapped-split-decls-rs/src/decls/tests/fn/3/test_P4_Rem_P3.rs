use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P4_Rem_P3 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type P1 = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type P4RemP3 = < < A as Rem < B > > :: Output as Same < P1 > > :: Output ; assert_eq ! (< P4RemP3 as Integer >:: to_i64 () , < P1 as Integer >:: to_i64 ()) ; }