use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N1_Sub_P5 () { type A = NInt < UInt < UTerm , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N6 = NInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N1SubP5 = < < A as Sub < B > > :: Output as Same < N6 > > :: Output ; assert_eq ! (< N1SubP5 as Integer >:: to_i64 () , < N6 as Integer >:: to_i64 ()) ; }