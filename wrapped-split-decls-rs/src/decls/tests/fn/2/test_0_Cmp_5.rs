use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_0_Cmp_5 () { type A = UTerm ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; # [allow (non_camel_case_types)] type U0CmpU5 = < A as Cmp < B > > :: Output ; assert_eq ! (< U0CmpU5 as Ord >:: to_ordering () , Ordering :: Less) ; }
}