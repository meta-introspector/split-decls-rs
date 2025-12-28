use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N4_Cmp_P5 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type N4CmpP5 = < A as Cmp < B > > :: Output ; assert_eq ! (< N4CmpP5 as Ord >:: to_ordering () , Ordering :: Less) ; }
}