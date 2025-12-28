use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_3_Cmp_3");
# [test] # [allow (non_snake_case)] fn test_3_Cmp_3 () { type A = UInt < UInt < UTerm , B1 > , B1 > ; type B = UInt < UInt < UTerm , B1 > , B1 > ; # [allow (non_camel_case_types)] type U3CmpU3 = < A as Cmp < B > > :: Output ; assert_eq ! (< U3CmpU3 as Ord >:: to_ordering () , Ordering :: Equal) ; }
}