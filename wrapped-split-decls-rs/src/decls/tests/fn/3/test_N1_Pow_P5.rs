use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N1_Pow_P5 () { type A = NInt < UInt < UTerm , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N1 = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type N1PowP5 = < < A as Pow < B > > :: Output as Same < N1 > > :: Output ; assert_eq ! (< N1PowP5 as Integer >:: to_i64 () , < N1 as Integer >:: to_i64 ()) ; }
}