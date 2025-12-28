use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P1_Max_P2 () { type A = PInt < UInt < UTerm , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type P2 = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type P1MaxP2 = < < A as Max < B > > :: Output as Same < P2 > > :: Output ; assert_eq ! (< P1MaxP2 as Integer >:: to_i64 () , < P2 as Integer >:: to_i64 ()) ; }
}