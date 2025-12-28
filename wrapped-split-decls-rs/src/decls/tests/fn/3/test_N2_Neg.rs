use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N2_Neg () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type P2 = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type NegN2 = < < A as Neg > :: Output as Same < P2 > > :: Output ; assert_eq ! (< NegN2 as Integer >:: to_i64 () , < P2 as Integer >:: to_i64 ()) ; }
}