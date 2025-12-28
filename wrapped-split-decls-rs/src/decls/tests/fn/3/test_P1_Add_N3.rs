use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P1_Add_N3 () { type A = PInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type N2 = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type P1AddN3 = < < A as Add < B > > :: Output as Same < N2 > > :: Output ; assert_eq ! (< P1AddN3 as Integer >:: to_i64 () , < N2 as Integer >:: to_i64 ()) ; }
}