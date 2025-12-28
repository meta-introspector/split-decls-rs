use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_0_Cmp_0 () { type A = UTerm ; type B = UTerm ; # [allow (non_camel_case_types)] type U0CmpU0 = < A as Cmp < B > > :: Output ; assert_eq ! (< U0CmpU0 as Ord >:: to_ordering () , Ordering :: Equal) ; }
}