use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_3_Pow_5 () { type A = UInt < UInt < UTerm , B1 > , B1 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type U243 = UInt < UInt < UInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B1 > , B1 > , B0 > , B0 > , B1 > , B1 > ; # [allow (non_camel_case_types)] type U3PowU5 = < < A as Pow < B > > :: Output as Same < U243 > > :: Output ; assert_eq ! (< U3PowU5 as Unsigned >:: to_u64 () , < U243 as Unsigned >:: to_u64 ()) ; }
}