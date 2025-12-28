use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P4_Cmp_N3 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type P4CmpN3 = < A as Cmp < B > > :: Output ; assert_eq ! (< P4CmpN3 as Ord >:: to_ordering () , Ordering :: Greater) ; }