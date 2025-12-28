use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P3_Pow_P4 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type P81 = PInt < UInt < UInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > , B0 > , B0 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type P3PowP4 = < < A as Pow < B > > :: Output as Same < P81 > > :: Output ; assert_eq ! (< P3PowP4 as Integer >:: to_i64 () , < P81 as Integer >:: to_i64 ()) ; }
}