use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N4_Min_P4 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type N4 = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type N4MinP4 = < < A as Min < B > > :: Output as Same < N4 > > :: Output ; assert_eq ! (< N4MinP4 as Integer >:: to_i64 () , < N4 as Integer >:: to_i64 ()) ; }