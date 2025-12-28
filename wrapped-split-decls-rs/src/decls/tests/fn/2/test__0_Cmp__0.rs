use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test__0_Cmp__0 () { type A = Z0 ; type B = Z0 ; # [allow (non_camel_case_types)] type _0Cmp_0 = < A as Cmp < B > > :: Output ; assert_eq ! (< _0Cmp_0 as Ord >:: to_ordering () , Ordering :: Equal) ; }
}