use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_4_Cmp_4");
# [test] # [allow (non_snake_case)] fn test_4_Cmp_4 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U4CmpU4 = < A as Cmp < B > > :: Output ; assert_eq ! (< U4CmpU4 as Ord >:: to_ordering () , Ordering :: Equal) ; }
}