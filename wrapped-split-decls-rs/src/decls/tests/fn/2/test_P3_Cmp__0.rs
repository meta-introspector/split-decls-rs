use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P3_Cmp__0 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = Z0 ; # [allow (non_camel_case_types)] type P3Cmp_0 = < A as Cmp < B > > :: Output ; assert_eq ! (< P3Cmp_0 as Ord >:: to_ordering () , Ordering :: Greater) ; }
}