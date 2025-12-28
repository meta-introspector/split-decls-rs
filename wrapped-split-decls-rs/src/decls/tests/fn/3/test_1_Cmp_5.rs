use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_1_Cmp_5");
# [test] # [allow (non_snake_case)] fn test_1_Cmp_5 () { type A = UInt < UTerm , B1 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; # [allow (non_camel_case_types)] type U1CmpU5 = < A as Cmp < B > > :: Output ; assert_eq ! (< U1CmpU5 as Ord >:: to_ordering () , Ordering :: Less) ; }
}