use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N1_Add_P2 () { type A = NInt < UInt < UTerm , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type P1 = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type N1AddP2 = < < A as Add < B > > :: Output as Same < P1 > > :: Output ; assert_eq ! (< N1AddP2 as Integer >:: to_i64 () , < P1 as Integer >:: to_i64 ()) ; }
}