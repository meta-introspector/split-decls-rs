use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_2_Min_5 () { type A = UInt < UInt < UTerm , B1 > , B0 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type U2 = UInt < UInt < UTerm , B1 > , B0 > ; # [allow (non_camel_case_types)] type U2MinU5 = < < A as Min < B > > :: Output as Same < U2 > > :: Output ; assert_eq ! (< U2MinU5 as Unsigned >:: to_u64 () , < U2 as Unsigned >:: to_u64 ()) ; }
}