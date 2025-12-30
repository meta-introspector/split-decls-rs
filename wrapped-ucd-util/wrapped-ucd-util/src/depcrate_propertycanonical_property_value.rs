// Generated macro for canonical_property_value (function)
macro_rules! Depcrate_propertycanonical_property_value {
() => {
// Module: crate::property
// Provides: {"canonical_property_value"}
// Dependencies: {}
# [doc = " Find the canonical property value for the given normalized property"] # [doc = " value."] # [doc = ""] # [doc = " The given property values should correspond to the values for the property"] # [doc = " under question, which can be found using `property_values`."] # [doc = ""] # [doc = " If no such property value exists, then `None` is returned."] # [doc = ""] # [doc = " The normalized property value must have been normalized according to"] # [doc = " UAX44 LM3, which can be done using `symbolic_name_normalize`."] pub fn canonical_property_value (property_values : PropertyValues , normalized_property_value : & str ,) -> Option < & 'static str > { canonical_property_name (property_values , normalized_property_value) }
};
}
