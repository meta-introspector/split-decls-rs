use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N3_Cmp_P4 () { type A = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type N3CmpP4 = < A as Cmp < B > > :: Output ; assert_eq ! (< N3CmpP4 as Ord >:: to_ordering () , Ordering :: Less) ; }