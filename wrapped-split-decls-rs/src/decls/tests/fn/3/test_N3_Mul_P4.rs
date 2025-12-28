use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N3_Mul_P4 () { type A = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type N12 = NInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type N3MulP4 = < < A as Mul < B > > :: Output as Same < N12 > > :: Output ; assert_eq ! (< N3MulP4 as Integer >:: to_i64 () , < N12 as Integer >:: to_i64 ()) ; }