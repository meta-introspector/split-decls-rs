use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_2_Pow_2 () { type A = UInt < UInt < UTerm , B1 > , B0 > ; type B = UInt < UInt < UTerm , B1 > , B0 > ; type U4 = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U2PowU2 = < < A as Pow < B > > :: Output as Same < U4 > > :: Output ; assert_eq ! (< U2PowU2 as Unsigned >:: to_u64 () , < U4 as Unsigned >:: to_u64 ()) ; }
}