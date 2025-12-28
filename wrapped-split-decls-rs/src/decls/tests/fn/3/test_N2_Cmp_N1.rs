use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_N2_Cmp_N1");
# [test] # [allow (non_snake_case)] fn test_N2_Cmp_N1 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type N2CmpN1 = < A as Cmp < B > > :: Output ; assert_eq ! (< N2CmpN1 as Ord >:: to_ordering () , Ordering :: Less) ; }
}