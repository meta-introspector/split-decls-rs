use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P4_Cmp_P3 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type P4CmpP3 = < A as Cmp < B > > :: Output ; assert_eq ! (< P4CmpP3 as Ord >:: to_ordering () , Ordering :: Greater) ; }