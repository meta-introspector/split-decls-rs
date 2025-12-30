// Generated macro for canonical_property_name (function)
macro_rules! Depcrate_propertycanonical_property_name {
() => {
// Module: crate::property
// Provides: {"canonical_property_name"}
// Dependencies: {}
# [doc = " Find the canonical property name for the given normalized property name."] # [doc = ""] # [doc = " If no such property exists, then `None` is returned."] # [doc = ""] # [doc = " The normalized property name must have been normalized according to"] # [doc = " UAX44 LM3, which can be done using `symbolic_name_normalize`."] pub fn canonical_property_name (property_table : PropertyTable , normalized_property_name : & str ,) -> Option < & 'static str > { property_table . binary_search_by_key (& normalized_property_name , | & (n , _) | n) . ok () . map (| i | property_table [i] . 1) }
};
}
