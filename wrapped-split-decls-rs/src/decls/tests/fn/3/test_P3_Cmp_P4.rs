use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_P3_Cmp_P4 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type P3CmpP4 = < A as Cmp < B > > :: Output ; assert_eq ! (< P3CmpP4 as Ord >:: to_ordering () , Ordering :: Less) ; }
}