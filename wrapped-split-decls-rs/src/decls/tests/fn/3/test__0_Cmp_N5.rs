use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test__0_Cmp_N5");
# [test] # [allow (non_snake_case)] fn test__0_Cmp_N5 () { type A = Z0 ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type _0CmpN5 = < A as Cmp < B > > :: Output ; assert_eq ! (< _0CmpN5 as Ord >:: to_ordering () , Ordering :: Greater) ; }
}