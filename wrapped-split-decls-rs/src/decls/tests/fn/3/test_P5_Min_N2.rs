use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P5_Min_N2 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type N2 = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type P5MinN2 = < < A as Min < B > > :: Output as Same < N2 > > :: Output ; assert_eq ! (< P5MinN2 as Integer >:: to_i64 () , < N2 as Integer >:: to_i64 ()) ; }