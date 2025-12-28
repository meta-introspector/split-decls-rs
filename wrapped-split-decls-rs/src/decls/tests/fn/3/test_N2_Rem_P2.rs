use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N2_Rem_P2 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type _0 = Z0 ; # [allow (non_camel_case_types)] type N2RemP2 = < < A as Rem < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< N2RemP2 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }