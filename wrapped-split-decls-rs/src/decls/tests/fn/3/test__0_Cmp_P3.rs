use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test__0_Cmp_P3");
# [test] # [allow (non_snake_case)] fn test__0_Cmp_P3 () { type A = Z0 ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type _0CmpP3 = < A as Cmp < B > > :: Output ; assert_eq ! (< _0CmpP3 as Ord >:: to_ordering () , Ordering :: Less) ; }
}