use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N5_Neg () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P5 = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type NegN5 = < < A as Neg > :: Output as Same < P5 > > :: Output ; assert_eq ! (< NegN5 as Integer >:: to_i64 () , < P5 as Integer >:: to_i64 ()) ; }