use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N4_Max_N1 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = NInt < UInt < UTerm , B1 > > ; type N1 = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type N4MaxN1 = < < A as Max < B > > :: Output as Same < N1 > > :: Output ; assert_eq ! (< N4MaxN1 as Integer >:: to_i64 () , < N1 as Integer >:: to_i64 ()) ; }
}