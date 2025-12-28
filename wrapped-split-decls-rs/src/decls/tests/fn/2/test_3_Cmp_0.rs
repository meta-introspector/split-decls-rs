use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_3_Cmp_0 () { type A = UInt < UInt < UTerm , B1 > , B1 > ; type B = UTerm ; # [allow (non_camel_case_types)] type U3CmpU0 = < A as Cmp < B > > :: Output ; assert_eq ! (< U3CmpU0 as Ord >:: to_ordering () , Ordering :: Greater) ; }
}