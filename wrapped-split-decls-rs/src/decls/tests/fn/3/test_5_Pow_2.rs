use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_5_Pow_2 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type B = UInt < UInt < UTerm , B1 > , B0 > ; type U25 = UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > , B0 > , B1 > ; # [allow (non_camel_case_types)] type U5PowU2 = < < A as Pow < B > > :: Output as Same < U25 > > :: Output ; assert_eq ! (< U5PowU2 as Unsigned >:: to_u64 () , < U25 as Unsigned >:: to_u64 ()) ; }
}