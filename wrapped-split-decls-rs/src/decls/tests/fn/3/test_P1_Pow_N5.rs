use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P1_Pow_N5 () { type A = PInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P1 = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type P1PowN5 = < < A as Pow < B > > :: Output as Same < P1 > > :: Output ; assert_eq ! (< P1PowN5 as Integer >:: to_i64 () , < P1 as Integer >:: to_i64 ()) ; }
}