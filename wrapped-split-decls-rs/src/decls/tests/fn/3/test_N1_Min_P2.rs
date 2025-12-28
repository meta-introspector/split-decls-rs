use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N1_Min_P2 () { type A = NInt < UInt < UTerm , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type N1 = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type N1MinP2 = < < A as Min < B > > :: Output as Same < N1 > > :: Output ; assert_eq ! (< N1MinP2 as Integer >:: to_i64 () , < N1 as Integer >:: to_i64 ()) ; }
}