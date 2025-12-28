use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N3_Pow_P2 () { type A = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type P9 = PInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type N3PowP2 = < < A as Pow < B > > :: Output as Same < P9 > > :: Output ; assert_eq ! (< N3PowP2 as Integer >:: to_i64 () , < P9 as Integer >:: to_i64 ()) ; }
}