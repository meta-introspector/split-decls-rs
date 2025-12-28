use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P1_Sub_N5 () { type A = PInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P6 = PInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > > ; # [allow (non_camel_case_types)] type P1SubN5 = < < A as Sub < B > > :: Output as Same < P6 > > :: Output ; assert_eq ! (< P1SubN5 as Integer >:: to_i64 () , < P6 as Integer >:: to_i64 ()) ; }