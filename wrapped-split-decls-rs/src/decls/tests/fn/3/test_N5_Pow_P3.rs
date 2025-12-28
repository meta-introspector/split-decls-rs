use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N5_Pow_P3 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type N125 = NInt < UInt < UInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B1 > , B1 > , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type N5PowP3 = < < A as Pow < B > > :: Output as Same < N125 > > :: Output ; assert_eq ! (< N5PowP3 as Integer >:: to_i64 () , < N125 as Integer >:: to_i64 ()) ; }