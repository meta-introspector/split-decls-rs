use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N3_Pow_P3 () { type A = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type N27 = NInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > , B1 > , B1 > > ; # [allow (non_camel_case_types)] type N3PowP3 = < < A as Pow < B > > :: Output as Same < N27 > > :: Output ; assert_eq ! (< N3PowP3 as Integer >:: to_i64 () , < N27 as Integer >:: to_i64 ()) ; }