// Generated macro for check_directory (function)
macro_rules! Depcratecheck_directory {
() => {
// Module: crate
// Provides: {"check_directory"}
// Dependencies: {}
fn check_directory (dir : & Path , bad : & mut bool) -> Result < () , Box < dyn Error > > { for entry in fs :: read_dir (dir) ? { let entry = entry ? ; let path = entry . path () ; if path . is_dir () { check_directory (& path , bad) ? ; continue ; } if ! matches ! (path . extension () . and_then (| p | p . to_str ()) , Some ("md") | Some ("html")) { style_error ! (bad , path , "expected only md or html in src") ; } let contents = fs :: read_to_string (& path) ? ; if contents . contains ("#![feature") { style_error ! (bad , path , "#![feature] attributes are not allowed") ; } if ! cfg ! (windows) && contents . contains ('\r') { style_error ! (bad , path , "CR characters not allowed, must use LF line endings") ; } if contents . contains ('\t') { style_error ! (bad , path , "tab characters not allowed, use spaces") ; } if ! contents . ends_with ('\n') { style_error ! (bad , path , "file must end with a newline") ; } if contents . contains ('\u{2013}') { style_error ! (bad , path , "en-dash not allowed, use two dashes like --") ; } if contents . contains ('\u{2014}') { style_error ! (bad , path , "em-dash not allowed, use three dashes like ---") ; } if contents . contains ('\u{a0}') { style_error ! (bad , path , "don't use 0xa0 no-break-space, use &nbsp; instead") ; } for line in contents . lines () { if line . ends_with (' ') { style_error ! (bad , path , "lines must not end with spaces") ; } } cmark_check (& path , bad , & contents) ? ; } Ok (()) }
};
}
