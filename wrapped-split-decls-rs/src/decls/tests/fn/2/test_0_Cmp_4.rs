use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_0_Cmp_4 () { type A = UTerm ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U0CmpU4 = < A as Cmp < B > > :: Output ; assert_eq ! (< U0CmpU4 as Ord >:: to_ordering () , Ordering :: Less) ; }
}