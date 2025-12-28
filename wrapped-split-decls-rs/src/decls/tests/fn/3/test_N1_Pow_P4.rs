use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N1_Pow_P4 () { type A = NInt < UInt < UTerm , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type P1 = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type N1PowP4 = < < A as Pow < B > > :: Output as Same < P1 > > :: Output ; assert_eq ! (< N1PowP4 as Integer >:: to_i64 () , < P1 as Integer >:: to_i64 ()) ; }
}