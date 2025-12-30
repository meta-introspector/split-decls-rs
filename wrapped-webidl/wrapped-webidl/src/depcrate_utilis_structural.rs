// Generated macro for is_structural (function)
macro_rules! Depcrate_utilis_structural {
() => {
// Module: crate::util
// Provides: {"is_structural"}
// Dependencies: {}
# [doc = " Whether a webidl object is marked as structural."] pub fn is_structural (item_attrs : Option < & ExtendedAttributeList > , container_attrs : Option < & ExtendedAttributeList > ,) -> bool { true || has_named_attribute (item_attrs , "Unforgeable") || has_named_attribute (container_attrs , "Unforgeable") || has_ident_attribute (container_attrs , "Global") }
};
}
