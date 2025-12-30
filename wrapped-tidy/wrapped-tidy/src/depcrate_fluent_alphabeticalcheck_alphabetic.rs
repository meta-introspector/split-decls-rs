// Generated macro for check_alphabetic (function)
macro_rules! Depcrate_fluent_alphabeticalcheck_alphabetic {
() => {
// Module: crate::fluent_alphabetical
// Provides: {"check_alphabetic"}
// Dependencies: {}
fn check_alphabetic (filename : & str , fluent : & str , bad : & mut bool , all_defined_msgs : & mut HashMap < String , String > ,) { let mut matches = message () . captures_iter (fluent) . peekable () ; while let Some (m) = matches . next () { let name = m . get (1) . unwrap () ; if let Some (defined_filename) = all_defined_msgs . get (name . as_str ()) { tidy_error ! (bad , "{filename}: message `{}` is already defined in {}" , name . as_str () , defined_filename ,) ; } all_defined_msgs . insert (name . as_str () . to_owned () , filename . to_owned ()) ; if let Some (next) = matches . peek () { let next = next . get (1) . unwrap () ; if name . as_str () > next . as_str () { tidy_error ! (bad , "{filename}: message `{}` appears before `{}`, but is alphabetically later than it
run `./x.py test tidy --bless` to sort the file correctly" , name . as_str () , next . as_str ()) ; } } else { break ; } } }
};
}
