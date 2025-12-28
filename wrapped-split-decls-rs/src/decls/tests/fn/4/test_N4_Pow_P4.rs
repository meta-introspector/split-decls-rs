use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N4_Pow_P4 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type P256 = PInt < UInt < UInt < UInt < UInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > , B0 > , B0 > , B0 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type N4PowP4 = < < A as Pow < B > > :: Output as Same < P256 > > :: Output ; assert_eq ! (< N4PowP4 as Integer >:: to_i64 () , < P256 as Integer >:: to_i64 ()) ; }
}