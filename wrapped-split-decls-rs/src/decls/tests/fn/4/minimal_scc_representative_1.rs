use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn minimal_scc_representative_1 () { let mut relation = TransitiveRelationBuilder :: default () ; relation . add ("a" , "c") ; relation . add ("c" , "d") ; relation . add ("d" , "e") ; relation . add ("e" , "c") ; relation . add ("b" , "d") ; relation . add ("b" , "e") ; let relation = relation . freeze () ; assert_eq ! (relation . minimal_scc_representative ("a") , "a") ; assert_eq ! (relation . minimal_scc_representative ("b") , "b") ; assert_eq ! (relation . minimal_scc_representative ("c") , "c") ; assert_eq ! (relation . minimal_scc_representative ("d") , "c") ; assert_eq ! (relation . minimal_scc_representative ("e") , "c") ; }
}