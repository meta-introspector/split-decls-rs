use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_3_Cmp_5");
# [test] # [allow (non_snake_case)] fn test_3_Cmp_5 () { type A = UInt < UInt < UTerm , B1 > , B1 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; # [allow (non_camel_case_types)] type U3CmpU5 = < A as Cmp < B > > :: Output ; assert_eq ! (< U3CmpU5 as Ord >:: to_ordering () , Ordering :: Less) ; }
}