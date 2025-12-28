use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N2_Pow_P1 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = PInt < UInt < UTerm , B1 > > ; type N2 = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N2PowP1 = < < A as Pow < B > > :: Output as Same < N2 > > :: Output ; assert_eq ! (< N2PowP1 as Integer >:: to_i64 () , < N2 as Integer >:: to_i64 ()) ; }
}