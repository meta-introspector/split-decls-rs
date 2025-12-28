use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P4_Cmp_P2 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type P4CmpP2 = < A as Cmp < B > > :: Output ; assert_eq ! (< P4CmpP2 as Ord >:: to_ordering () , Ordering :: Greater) ; }
}