// Generated macro for DuplicateLangItem (struct)
macro_rules! Depcrate_errorsDuplicateLangItem {
() => {
// Module: crate::errors
// Provides: {"DuplicateLangItem"}
// Dependencies: {}
pub (crate) struct DuplicateLangItem { pub local_span : Option < Span > , pub lang_item_name : Symbol , pub crate_name : Symbol , pub dependency_of : Option < Symbol > , pub is_local : bool , pub path : String , pub first_defined_span : Option < Span > , pub orig_crate_name : Option < Symbol > , pub orig_dependency_of : Option < Symbol > , pub orig_is_local : bool , pub orig_path : String , pub (crate) duplicate : Duplicate , }
};
}
