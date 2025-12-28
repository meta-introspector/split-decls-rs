use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_0_Cmp_1 () { type A = UTerm ; type B = UInt < UTerm , B1 > ; # [allow (non_camel_case_types)] type U0CmpU1 = < A as Cmp < B > > :: Output ; assert_eq ! (< U0CmpU1 as Ord >:: to_ordering () , Ordering :: Less) ; }
}