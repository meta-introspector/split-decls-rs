use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P4_Cmp__0 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = Z0 ; # [allow (non_camel_case_types)] type P4Cmp_0 = < A as Cmp < B > > :: Output ; assert_eq ! (< P4Cmp_0 as Ord >:: to_ordering () , Ordering :: Greater) ; }