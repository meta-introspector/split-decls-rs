use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P3_Cmp_N1 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type P3CmpN1 = < A as Cmp < B > > :: Output ; assert_eq ! (< P3CmpN1 as Ord >:: to_ordering () , Ordering :: Greater) ; }