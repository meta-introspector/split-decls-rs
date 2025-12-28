use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn mubs_triangle () { let mut relation = TransitiveRelationBuilder :: default () ; relation . add ("a" , "tcx") ; relation . add ("b" , "tcx") ; let relation = relation . freeze () ; assert_eq ! (relation . minimal_upper_bounds ("a" , "b") , vec ! ["tcx"]) ; assert_eq ! (relation . parents ("a") , vec ! ["tcx"]) ; assert_eq ! (relation . parents ("b") , vec ! ["tcx"]) ; }
}