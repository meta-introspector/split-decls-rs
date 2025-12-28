use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_2_Pow_4 () { type A = UInt < UInt < UTerm , B1 > , B0 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type U16 = UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U2PowU4 = < < A as Pow < B > > :: Output as Same < U16 > > :: Output ; assert_eq ! (< U2PowU4 as Unsigned >:: to_u64 () , < U16 as Unsigned >:: to_u64 ()) ; }
}