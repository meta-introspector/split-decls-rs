// Generated macro for tests (module)
macro_rules! Depcrate_propertytests {
() => {
// Module: crate::property
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: unicode_tables :: property_names :: PROPERTY_NAMES ; use crate :: unicode_tables :: property_values :: PROPERTY_VALUES ; use super :: { canonical_property_name , canonical_property_value , property_values , } ; # [test] fn canonical_property_name_1 () { assert_eq ! (canonical_property_name (PROPERTY_NAMES , "gc") , Some ("General_Category")) ; assert_eq ! (canonical_property_name (PROPERTY_NAMES , "generalcategory") , Some ("General_Category")) ; assert_eq ! (canonical_property_name (PROPERTY_NAMES , "g c") , None) ; } # [test] fn property_values_1 () { assert_eq ! (property_values (PROPERTY_VALUES , "White_Space") , Some (& [("f" , "No") , ("false" , "No") , ("n" , "No") , ("no" , "No") , ("t" , "Yes") , ("true" , "Yes") , ("y" , "Yes") , ("yes" , "Yes") ,] [..])) ; } # [test] fn canonical_property_value_1 () { let values = property_values (PROPERTY_VALUES , "White_Space") . unwrap () ; assert_eq ! (canonical_property_value (values , "false") , Some ("No")) ; assert_eq ! (canonical_property_value (values , "t") , Some ("Yes")) ; assert_eq ! (canonical_property_value (values , "F") , None) ; } }
};
}
