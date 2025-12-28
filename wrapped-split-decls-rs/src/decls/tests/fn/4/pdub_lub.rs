use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn pdub_lub () { let mut relation = TransitiveRelationBuilder :: default () ; relation . add ("a" , "a1") ; relation . add ("b" , "b1") ; relation . add ("a1" , "x") ; relation . add ("b1" , "x") ; let relation = relation . freeze () ; assert_eq ! (relation . minimal_upper_bounds ("a" , "b") , vec ! ["x"]) ; assert_eq ! (relation . postdom_upper_bound ("a" , "b") , Some ("x")) ; assert_eq ! (relation . postdom_parent ("a") , Some ("a1")) ; assert_eq ! (relation . postdom_parent ("b") , Some ("b1")) ; assert_eq ! (relation . postdom_parent ("a1") , Some ("x")) ; assert_eq ! (relation . postdom_parent ("b1") , Some ("x")) ; }
}