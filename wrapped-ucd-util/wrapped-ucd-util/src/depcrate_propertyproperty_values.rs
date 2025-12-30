// Generated macro for property_values (function)
macro_rules! Depcrate_propertyproperty_values {
() => {
// Module: crate::property
// Provides: {"property_values"}
// Dependencies: {}
# [doc = " Find the set of possible property values for a given property."] # [doc = ""] # [doc = " The set returned is a mapping expressed as a sorted list of tuples."] # [doc = " The first element of each tuple is a normalized property value while the"] # [doc = " second element of each tuple is the corresponding canonical property"] # [doc = " value."] # [doc = ""] # [doc = " If no such property exists, then `None` is returned."] # [doc = ""] # [doc = " The given property name must be in its canonical form, which can be"] # [doc = " found using `canonical_property_name`."] pub fn property_values (property_value_table : PropertyValueTable , canonical_property_name : & str ,) -> Option < PropertyValues > { property_value_table . binary_search_by_key (& canonical_property_name , | & (n , _) | n) . ok () . map (| i | property_value_table [i] . 1) }
};
}
