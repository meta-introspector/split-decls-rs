use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N3_Mul_N3 () { type A = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type P9 = PInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type N3MulN3 = < < A as Mul < B > > :: Output as Same < P9 > > :: Output ; assert_eq ! (< N3MulN3 as Integer >:: to_i64 () , < P9 as Integer >:: to_i64 ()) ; }
}