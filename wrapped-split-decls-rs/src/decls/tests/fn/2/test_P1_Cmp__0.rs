use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P1_Cmp__0 () { type A = PInt < UInt < UTerm , B1 > > ; type B = Z0 ; # [allow (non_camel_case_types)] type P1Cmp_0 = < A as Cmp < B > > :: Output ; assert_eq ! (< P1Cmp_0 as Ord >:: to_ordering () , Ordering :: Greater) ; }