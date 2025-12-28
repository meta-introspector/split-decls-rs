use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P5_Cmp__0 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = Z0 ; # [allow (non_camel_case_types)] type P5Cmp_0 = < A as Cmp < B > > :: Output ; assert_eq ! (< P5Cmp_0 as Ord >:: to_ordering () , Ordering :: Greater) ; }
}