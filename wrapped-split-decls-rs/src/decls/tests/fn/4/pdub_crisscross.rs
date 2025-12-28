use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn pdub_crisscross () { let mut relation = TransitiveRelationBuilder :: default () ; relation . add ("a" , "a1") ; relation . add ("a" , "b1") ; relation . add ("b" , "a1") ; relation . add ("b" , "b1") ; relation . add ("a1" , "x") ; relation . add ("b1" , "x") ; let relation = relation . freeze () ; assert_eq ! (relation . minimal_upper_bounds ("a" , "b") , vec ! ["a1" , "b1"]) ; assert_eq ! (relation . postdom_upper_bound ("a" , "b") , Some ("x")) ; assert_eq ! (relation . postdom_parent ("a") , Some ("x")) ; assert_eq ! (relation . postdom_parent ("b") , Some ("x")) ; }
}