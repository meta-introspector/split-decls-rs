use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P5_Cmp_P1 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type P5CmpP1 = < A as Cmp < B > > :: Output ; assert_eq ! (< P5CmpP1 as Ord >:: to_ordering () , Ordering :: Greater) ; }