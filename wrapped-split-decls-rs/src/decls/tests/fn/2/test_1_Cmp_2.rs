use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_1_Cmp_2 () { type A = UInt < UTerm , B1 > ; type B = UInt < UInt < UTerm , B1 > , B0 > ; # [allow (non_camel_case_types)] type U1CmpU2 = < A as Cmp < B > > :: Output ; assert_eq ! (< U1CmpU2 as Ord >:: to_ordering () , Ordering :: Less) ; }
}