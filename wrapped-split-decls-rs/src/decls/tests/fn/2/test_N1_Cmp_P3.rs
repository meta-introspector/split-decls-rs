use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N1_Cmp_P3 () { type A = NInt < UInt < UTerm , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type N1CmpP3 = < A as Cmp < B > > :: Output ; assert_eq ! (< N1CmpP3 as Ord >:: to_ordering () , Ordering :: Less) ; }