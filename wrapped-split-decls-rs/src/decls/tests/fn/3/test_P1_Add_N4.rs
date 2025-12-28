use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P1_Add_N4 () { type A = PInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type N3 = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type P1AddN4 = < < A as Add < B > > :: Output as Same < N3 > > :: Output ; assert_eq ! (< P1AddN4 as Integer >:: to_i64 () , < N3 as Integer >:: to_i64 ()) ; }
}