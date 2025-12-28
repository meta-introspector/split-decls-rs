use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_1_Cmp_1");
# [test] # [allow (non_snake_case)] fn test_1_Cmp_1 () { type A = UInt < UTerm , B1 > ; type B = UInt < UTerm , B1 > ; # [allow (non_camel_case_types)] type U1CmpU1 = < A as Cmp < B > > :: Output ; assert_eq ! (< U1CmpU1 as Ord >:: to_ordering () , Ordering :: Equal) ; }
}