// Generated macro for parse_opts (function)
macro_rules! Depcrate_cliparse_opts {
() => {
// Module: crate::cli
// Provides: {"parse_opts"}
// Dependencies: {}
# [doc = " Parses command line arguments into test options."] # [doc = " Returns `None` if help was requested (since we only show help message and don't run tests),"] # [doc = " returns `Some(Err(..))` if provided arguments are incorrect,"] # [doc = " otherwise creates a `TestOpts` object and returns it."] pub fn parse_opts (args : & [String]) -> Option < OptRes > { let mut opts = optgroups () ; opts . optflag ("" , "nocapture" , "Deprecated, use `--no-capture`") ; let binary = args . first () . map (| c | & * * c) . unwrap_or ("...") ; let args = args . get (1 ..) . unwrap_or (args) ; let matches = match opts . parse (args) { Ok (m) => m , Err (f) => return Some (Err (f . to_string ())) , } ; if matches . opt_present ("h") { usage (binary , & optgroups ()) ; return None ; } let opts_result = parse_opts_impl (matches) ; Some (opts_result) }
};
}
