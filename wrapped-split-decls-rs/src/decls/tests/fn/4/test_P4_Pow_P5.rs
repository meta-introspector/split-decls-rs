use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P4_Pow_P5 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P1024 = PInt < UInt < UInt < UInt < UInt < UInt < UInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > , B0 > , B0 > , B0 > , B0 > , B0 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type P4PowP5 = < < A as Pow < B > > :: Output as Same < P1024 > > :: Output ; assert_eq ! (< P4PowP5 as Integer >:: to_i64 () , < P1024 as Integer >:: to_i64 ()) ; }
}