use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N3_Min_P3 () { type A = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type N3 = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type N3MinP3 = < < A as Min < B > > :: Output as Same < N3 > > :: Output ; assert_eq ! (< N3MinP3 as Integer >:: to_i64 () , < N3 as Integer >:: to_i64 ()) ; }
}