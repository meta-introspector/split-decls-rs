use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_N5_Add_N4");
# [test] # [allow (non_snake_case)] fn test_N5_Add_N4 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type N9 = NInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type N5AddN4 = < < A as Add < B > > :: Output as Same < N9 > > :: Output ; assert_eq ! (< N5AddN4 as Integer >:: to_i64 () , < N9 as Integer >:: to_i64 ()) ; }
}