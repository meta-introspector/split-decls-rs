use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P1_Max_N1 () { type A = PInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UTerm , B1 > > ; type P1 = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type P1MaxN1 = < < A as Max < B > > :: Output as Same < P1 > > :: Output ; assert_eq ! (< P1MaxN1 as Integer >:: to_i64 () , < P1 as Integer >:: to_i64 ()) ; }
}