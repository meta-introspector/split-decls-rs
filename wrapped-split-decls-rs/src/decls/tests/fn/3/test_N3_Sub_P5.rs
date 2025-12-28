use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N3_Sub_P5 () { type A = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N8 = NInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type N3SubP5 = < < A as Sub < B > > :: Output as Same < N8 > > :: Output ; assert_eq ! (< N3SubP5 as Integer >:: to_i64 () , < N8 as Integer >:: to_i64 ()) ; }