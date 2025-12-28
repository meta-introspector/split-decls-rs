use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P2_Cmp_N4 () { type A = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type P2CmpN4 = < A as Cmp < B > > :: Output ; assert_eq ! (< P2CmpN4 as Ord >:: to_ordering () , Ordering :: Greater) ; }
}