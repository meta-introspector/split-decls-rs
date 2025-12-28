use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P4_Rem_N1 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = NInt < UInt < UTerm , B1 > > ; type _0 = Z0 ; # [allow (non_camel_case_types)] type P4RemN1 = < < A as Rem < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< P4RemN1 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }