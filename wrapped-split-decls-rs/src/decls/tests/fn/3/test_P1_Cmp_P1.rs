use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_P1_Cmp_P1");
# [test] # [allow (non_snake_case)] fn test_P1_Cmp_P1 () { type A = PInt < UInt < UTerm , B1 > > ; type B = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type P1CmpP1 = < A as Cmp < B > > :: Output ; assert_eq ! (< P1CmpP1 as Ord >:: to_ordering () , Ordering :: Equal) ; }
}