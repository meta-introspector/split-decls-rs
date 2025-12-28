use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: minimal_scc_representative_2");
# [test] fn minimal_scc_representative_2 () { let mut relation = TransitiveRelationBuilder :: default () ; relation . add ("a" , "b") ; relation . add ("b" , "a") ; relation . add ("a" , "a") ; relation . add ("c" , "c") ; let relation = relation . freeze () ; assert_eq ! (relation . minimal_scc_representative ("a") , "a") ; assert_eq ! (relation . minimal_scc_representative ("b") , "a") ; assert_eq ! (relation . minimal_scc_representative ("c") , "c") ; }
}