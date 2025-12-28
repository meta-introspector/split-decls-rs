use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N4_Pow_P1 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = PInt < UInt < UTerm , B1 > > ; type N4 = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type N4PowP1 = < < A as Pow < B > > :: Output as Same < N4 > > :: Output ; assert_eq ! (< N4PowP1 as Integer >:: to_i64 () , < N4 as Integer >:: to_i64 ()) ; }
}