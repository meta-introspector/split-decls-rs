use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn test_one_step () { let mut relation = TransitiveRelationBuilder :: default () ; relation . add ("a" , "b") ; relation . add ("a" , "c") ; let relation = relation . freeze () ; assert ! (relation . contains ("a" , "c")) ; assert ! (relation . contains ("a" , "b")) ; assert ! (! relation . contains ("b" , "a")) ; assert ! (! relation . contains ("a" , "d")) ; }
}