use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N4_Max_N3 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type N3 = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type N4MaxN3 = < < A as Max < B > > :: Output as Same < N3 > > :: Output ; assert_eq ! (< N4MaxN3 as Integer >:: to_i64 () , < N3 as Integer >:: to_i64 ()) ; }
}