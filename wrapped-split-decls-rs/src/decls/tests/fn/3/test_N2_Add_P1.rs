use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N2_Add_P1 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = PInt < UInt < UTerm , B1 > > ; type N1 = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type N2AddP1 = < < A as Add < B > > :: Output as Same < N1 > > :: Output ; assert_eq ! (< N2AddP1 as Integer >:: to_i64 () , < N1 as Integer >:: to_i64 ()) ; }
}