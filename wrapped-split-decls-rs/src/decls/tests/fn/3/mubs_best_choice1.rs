use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn mubs_best_choice1 () { let mut relation = TransitiveRelationBuilder :: default () ; relation . add ("0" , "1") ; relation . add ("0" , "2") ; relation . add ("2" , "1") ; relation . add ("3" , "1") ; relation . add ("3" , "2") ; let relation = relation . freeze () ; assert_eq ! (relation . minimal_upper_bounds ("0" , "3") , vec ! ["2"]) ; assert_eq ! (relation . parents ("0") , vec ! ["2"]) ; assert_eq ! (relation . parents ("2") , vec ! ["1"]) ; assert ! (relation . parents ("1") . is_empty ()) ; }
}