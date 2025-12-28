use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn mubs_best_choice_scc () { let mut relation = TransitiveRelationBuilder :: default () ; relation . add ("0" , "1") ; relation . add ("0" , "2") ; relation . add ("1" , "2") ; relation . add ("2" , "1") ; relation . add ("3" , "1") ; relation . add ("3" , "2") ; let relation = relation . freeze () ; assert_eq ! (relation . minimal_upper_bounds ("0" , "3") , vec ! ["1"]) ; assert_eq ! (relation . parents ("0") , vec ! ["1"]) ; }