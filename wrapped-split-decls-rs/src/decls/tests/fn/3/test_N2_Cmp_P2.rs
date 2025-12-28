use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_N2_Cmp_P2");
# [test] # [allow (non_snake_case)] fn test_N2_Cmp_P2 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N2CmpP2 = < A as Cmp < B > > :: Output ; assert_eq ! (< N2CmpP2 as Ord >:: to_ordering () , Ordering :: Less) ; }
}