// Generated macro for require_path_only (function)
macro_rules! Depcrate_attr_parse_metarequire_path_only {
() => {
// Module: crate::attr::parse_meta
// Provides: {"require_path_only"}
// Dependencies: {}
pub fn require_path_only (meta : & CustomMeta , cx : & AttrCtxt) -> Result < () , () > { let error_args = | | { format ! ("unexpected value of {} {} attribute item" , cx . attr_type , path_str (meta . path ())) } ; match & meta { CustomMeta :: Path (_) => Ok (()) , CustomMeta :: List (meta) => { cx . syn_error (syn :: Error :: new (meta . delimiter . span () . join () , error_args ())) ; Err (()) } CustomMeta :: NameValue (meta) => { let eq_token = & meta . eq_token ; let value = & meta . value ; cx . error_spanned_by (quote ! (# eq_token # value) , error_args ()) ; Err (()) } CustomMeta :: Not (..) => { Err (()) } } }
};
}
