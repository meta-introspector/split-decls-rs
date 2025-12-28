use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N3_Mul_N5 () { type A = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P15 = PInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B1 > , B1 > > ; # [allow (non_camel_case_types)] type N3MulN5 = < < A as Mul < B > > :: Output as Same < P15 > > :: Output ; assert_eq ! (< N3MulN5 as Integer >:: to_i64 () , < P15 as Integer >:: to_i64 ()) ; }