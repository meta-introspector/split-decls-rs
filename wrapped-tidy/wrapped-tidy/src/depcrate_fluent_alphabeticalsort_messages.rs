// Generated macro for sort_messages (function)
macro_rules! Depcrate_fluent_alphabeticalsort_messages {
() => {
// Module: crate::fluent_alphabetical
// Provides: {"sort_messages"}
// Dependencies: {}
fn sort_messages (filename : & str , fluent : & str , bad : & mut bool , all_defined_msgs : & mut HashMap < String , String > ,) -> String { let mut chunks = vec ! [] ; let mut cur = String :: new () ; for line in fluent . lines () { if let Some (name) = message () . find (line) { if let Some (defined_filename) = all_defined_msgs . get (name . as_str ()) { tidy_error ! (bad , "{filename}: message `{}` is already defined in {}" , name . as_str () , defined_filename ,) ; } all_defined_msgs . insert (name . as_str () . to_owned () , filename . to_owned ()) ; chunks . push (std :: mem :: take (& mut cur)) ; } cur += line ; cur . push ('\n') ; } chunks . push (cur) ; chunks . sort () ; let mut out = chunks . join ("") ; out = out . trim () . to_string () ; out . push ('\n') ; out }
};
}
