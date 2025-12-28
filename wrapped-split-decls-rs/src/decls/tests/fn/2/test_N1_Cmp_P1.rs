use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N1_Cmp_P1 () { type A = NInt < UInt < UTerm , B1 > > ; type B = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type N1CmpP1 = < A as Cmp < B > > :: Output ; assert_eq ! (< N1CmpP1 as Ord >:: to_ordering () , Ordering :: Less) ; }
}