use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P3_Cmp_N3 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type P3CmpN3 = < A as Cmp < B > > :: Output ; assert_eq ! (< P3CmpN3 as Ord >:: to_ordering () , Ordering :: Greater) ; }