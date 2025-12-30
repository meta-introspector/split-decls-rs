// Generated macro for get_format (function)
macro_rules! Depcrate_cliget_format {
() => {
// Module: crate::cli
// Provides: {"get_format"}
// Dependencies: {}
fn get_format (matches : & getopts :: Matches , quiet : bool , allow_unstable : bool ,) -> OptPartRes < OutputFormat > { let format = match matches . opt_str ("format") . as_deref () { None if quiet => OutputFormat :: Terse , Some ("pretty") | None => OutputFormat :: Pretty , Some ("terse") => OutputFormat :: Terse , Some ("json") => { if ! allow_unstable { return Err ("The \"json\" format is only accepted on the nightly compiler with -Z unstable-options" . into ()) ; } OutputFormat :: Json } Some ("junit") => { if ! allow_unstable { return Err ("The \"junit\" format is only accepted on the nightly compiler with -Z unstable-options" . into ()) ; } OutputFormat :: Junit } Some (v) => { return Err (format ! ("argument for --format must be pretty, terse, json or junit (was \
                 {v})")) ; } } ; Ok (format) }
};
}
