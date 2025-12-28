use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_N5_Cmp_P1");
# [test] # [allow (non_snake_case)] fn test_N5_Cmp_P1 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type N5CmpP1 = < A as Cmp < B > > :: Output ; assert_eq ! (< N5CmpP1 as Ord >:: to_ordering () , Ordering :: Less) ; }
}