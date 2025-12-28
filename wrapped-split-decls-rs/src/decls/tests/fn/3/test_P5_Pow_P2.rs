use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P5_Pow_P2 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type P25 = PInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type P5PowP2 = < < A as Pow < B > > :: Output as Same < P25 > > :: Output ; assert_eq ! (< P5PowP2 as Integer >:: to_i64 () , < P25 as Integer >:: to_i64 ()) ; }