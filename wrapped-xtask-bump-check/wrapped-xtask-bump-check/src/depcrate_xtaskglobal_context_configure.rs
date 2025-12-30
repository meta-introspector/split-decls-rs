// Generated macro for global_context_configure (function)
macro_rules! Depcrate_xtaskglobal_context_configure {
() => {
// Module: crate::xtask
// Provides: {"global_context_configure"}
// Dependencies: {}
fn global_context_configure (gctx : & mut GlobalContext , args : & ArgMatches) -> CliResult { let verbose = args . verbose () ; let quiet = args . flag ("quiet") ; let color = args . get_one :: < String > ("color") . map (String :: as_str) ; let frozen = args . flag ("frozen") ; let locked = args . flag ("locked") ; let offline = args . flag ("offline") ; let mut unstable_flags = vec ! [] ; if let Some (values) = args . get_many :: < String > ("unstable-features") { unstable_flags . extend (values . cloned ()) ; } let mut config_args = vec ! [] ; if let Some (values) = args . get_many :: < String > ("config") { config_args . extend (values . cloned ()) ; } gctx . configure (verbose , quiet , color , frozen , locked , offline , & None , & unstable_flags , & config_args ,) ? ; Ok (()) }
};
}
