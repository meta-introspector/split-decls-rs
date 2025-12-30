// Generated macro for check (function)
macro_rules! Depcrate_extra_checkscheck {
() => {
// Module: crate::extra_checks
// Provides: {"check"}
// Dependencies: {}
pub fn check (root_path : & Path , outdir : & Path , ci_info : & CiInfo , librustdoc_path : & Path , tools_path : & Path , npm : & Path , cargo : & Path , bless : bool , extra_checks : Option < & str > , pos_args : & [String] , bad : & mut bool ,) { if let Err (e) = check_impl (root_path , outdir , ci_info , librustdoc_path , tools_path , npm , cargo , bless , extra_checks , pos_args ,) { tidy_error ! (bad , "{e}") ; } }
};
}
