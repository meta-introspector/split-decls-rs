use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P2_Cmp_N2 () { type A = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type P2CmpN2 = < A as Cmp < B > > :: Output ; assert_eq ! (< P2CmpN2 as Ord >:: to_ordering () , Ordering :: Greater) ; }