use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N5_Min_P3 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type N5 = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type N5MinP3 = < < A as Min < B > > :: Output as Same < N5 > > :: Output ; assert_eq ! (< N5MinP3 as Integer >:: to_i64 () , < N5 as Integer >:: to_i64 ()) ; }
}