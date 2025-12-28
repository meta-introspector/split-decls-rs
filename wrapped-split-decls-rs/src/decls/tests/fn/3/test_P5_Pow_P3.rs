use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P5_Pow_P3 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type P125 = PInt < UInt < UInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B1 > , B1 > , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type P5PowP3 = < < A as Pow < B > > :: Output as Same < P125 > > :: Output ; assert_eq ! (< P5PowP3 as Integer >:: to_i64 () , < P125 as Integer >:: to_i64 ()) ; }
}