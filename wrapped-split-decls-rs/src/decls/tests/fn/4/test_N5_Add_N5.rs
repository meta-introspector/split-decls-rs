use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_N5_Add_N5");
# [test] # [allow (non_snake_case)] fn test_N5_Add_N5 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N10 = NInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N5AddN5 = < < A as Add < B > > :: Output as Same < N10 > > :: Output ; assert_eq ! (< N5AddN5 as Integer >:: to_i64 () , < N10 as Integer >:: to_i64 ()) ; }
}