// Generated macro for get_base_commit (function)
macro_rules! Depcrate_xtaskget_base_commit {
() => {
// Module: crate::xtask
// Provides: {"get_base_commit"}
// Dependencies: {}
# [doc = " Returns the commit of upstream `master` branch if `base-rev` is missing."] fn get_base_commit < 'a > (gctx : & GlobalContext , args : & clap :: ArgMatches , repo : & 'a git2 :: Repository ,) -> CargoResult < git2 :: Commit < 'a > > { let base_commit = match args . get_one :: < String > ("base-rev") { Some (sha) => { let obj = repo . revparse_single (sha) ? ; obj . peel_to_commit () ? } None => { let upstream_branches = repo . branches (Some (git2 :: BranchType :: Remote)) ? . filter_map (| r | r . ok ()) . filter (| (b , _) | { b . name () . ok () . flatten () . unwrap_or_default () . ends_with (& format ! ("/{UPSTREAM_BRANCH}")) }) . map (| (b , _) | b) . collect :: < Vec < _ > > () ; if upstream_branches . is_empty () { anyhow :: bail ! ("could not find `base-sha` for `{UPSTREAM_BRANCH}`, pass it in directly") ; } let upstream_ref = upstream_branches [0] . get () ; if upstream_branches . len () > 1 { let name = upstream_ref . name () . expect ("name is valid UTF-8") ; let _ = gctx . shell () . warn (format ! ("multiple `{UPSTREAM_BRANCH}` found, picking {name}")) ; } upstream_ref . peel_to_commit () ? } } ; Ok (base_commit) }
};
}
