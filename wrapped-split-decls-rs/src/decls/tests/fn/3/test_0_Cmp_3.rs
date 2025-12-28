use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_0_Cmp_3");
# [test] # [allow (non_snake_case)] fn test_0_Cmp_3 () { type A = UTerm ; type B = UInt < UInt < UTerm , B1 > , B1 > ; # [allow (non_camel_case_types)] type U0CmpU3 = < A as Cmp < B > > :: Output ; assert_eq ! (< U0CmpU3 as Ord >:: to_ordering () , Ordering :: Less) ; }
}