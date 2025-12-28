use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N5_Max_N5 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N5 = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type N5MaxN5 = < < A as Max < B > > :: Output as Same < N5 > > :: Output ; assert_eq ! (< N5MaxN5 as Integer >:: to_i64 () , < N5 as Integer >:: to_i64 ()) ; }
}