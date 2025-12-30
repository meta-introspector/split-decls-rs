// Generated macro for target_value (macro)
macro_rules! Depcrate_format_description_parse_format_itemtarget_value {
() => {
// Module: crate::format_description::parse::format_item
// Provides: {"target_value"}
// Dependencies: {}
# [doc = " Get the target value for a given enum."] macro_rules ! target_value { ($ name : ident $ variant : ident $ value : expr) => { $ value } ; ($ name : ident $ variant : ident) => { $ crate :: format_description :: modifier ::$ name ::$ variant } ; }
};
}
