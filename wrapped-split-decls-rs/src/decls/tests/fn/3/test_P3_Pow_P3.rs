use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P3_Pow_P3 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type P27 = PInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > , B1 > , B1 > > ; # [allow (non_camel_case_types)] type P3PowP3 = < < A as Pow < B > > :: Output as Same < P27 > > :: Output ; assert_eq ! (< P3PowP3 as Integer >:: to_i64 () , < P27 as Integer >:: to_i64 ()) ; }