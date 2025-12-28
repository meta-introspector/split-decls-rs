use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_5_Rem_3 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type B = UInt < UInt < UTerm , B1 > , B1 > ; type U2 = UInt < UInt < UTerm , B1 > , B0 > ; # [allow (non_camel_case_types)] type U5RemU3 = < < A as Rem < B > > :: Output as Same < U2 > > :: Output ; assert_eq ! (< U5RemU3 as Unsigned >:: to_u64 () , < U2 as Unsigned >:: to_u64 ()) ; }
}