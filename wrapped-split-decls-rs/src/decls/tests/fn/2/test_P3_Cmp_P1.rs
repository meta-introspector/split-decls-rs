use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P3_Cmp_P1 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type P3CmpP1 = < A as Cmp < B > > :: Output ; assert_eq ! (< P3CmpP1 as Ord >:: to_ordering () , Ordering :: Greater) ; }
}