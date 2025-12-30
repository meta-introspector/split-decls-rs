// Generated macro for register_metavariable (function)
macro_rules! Depcrate_macrosregister_metavariable {
() => {
// Module: crate::macros
// Provides: {"register_metavariable"}
// Dependencies: {}
fn register_metavariable (map : & mut HashMap < String , String > , result : & mut String , name : & str , dollar_count : usize ,) { let mut new_name = "$" . repeat (dollar_count - 1) ; let mut old_name = "$" . repeat (dollar_count) ; new_name . push ('z') ; new_name . push_str (name) ; old_name . push_str (name) ; result . push_str (& new_name) ; map . insert (old_name , new_name) ; }
};
}
