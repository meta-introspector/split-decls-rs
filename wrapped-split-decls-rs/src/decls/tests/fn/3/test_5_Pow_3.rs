use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_5_Pow_3 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type B = UInt < UInt < UTerm , B1 > , B1 > ; type U125 = UInt < UInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B1 > , B1 > , B1 > , B0 > , B1 > ; # [allow (non_camel_case_types)] type U5PowU3 = < < A as Pow < B > > :: Output as Same < U125 > > :: Output ; assert_eq ! (< U5PowU3 as Unsigned >:: to_u64 () , < U125 as Unsigned >:: to_u64 ()) ; }
}