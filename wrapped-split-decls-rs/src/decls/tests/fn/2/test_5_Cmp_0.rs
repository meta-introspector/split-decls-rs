use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_5_Cmp_0 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type B = UTerm ; # [allow (non_camel_case_types)] type U5CmpU0 = < A as Cmp < B > > :: Output ; assert_eq ! (< U5CmpU0 as Ord >:: to_ordering () , Ordering :: Greater) ; }
}