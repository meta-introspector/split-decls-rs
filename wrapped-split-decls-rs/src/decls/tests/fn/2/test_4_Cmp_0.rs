use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_4_Cmp_0 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type B = UTerm ; # [allow (non_camel_case_types)] type U4CmpU0 = < A as Cmp < B > > :: Output ; assert_eq ! (< U4CmpU0 as Ord >:: to_ordering () , Ordering :: Greater) ; }
}