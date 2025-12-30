// Generated macro for filter_used_messages (function)
macro_rules! Depcrate_fluent_usedfilter_used_messages {
() => {
// Module: crate::fluent_used
// Provides: {"filter_used_messages"}
// Dependencies: {}
fn filter_used_messages (contents : & str , msgs_not_appeared_yet : & mut HashMap < String , String > , msgs_appeared_only_once : & mut HashMap < String , String > ,) { let matches = static_regex ! (r"\w+") . find_iter (contents) ; for name in matches { if let Some ((name , filename)) = msgs_not_appeared_yet . remove_entry (name . as_str ()) { msgs_appeared_only_once . insert (name , filename) ; } else { msgs_appeared_only_once . remove (name . as_str ()) ; } } }
};
}
