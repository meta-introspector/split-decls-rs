// Generated macro for get_head_commit (function)
macro_rules! Depcrate_xtaskget_head_commit {
() => {
// Module: crate::xtask
// Provides: {"get_head_commit"}
// Dependencies: {}
# [doc = " Returns `HEAD` of the Git repository if `head-rev` is missing."] fn get_head_commit < 'a > (args : & clap :: ArgMatches , repo : & 'a git2 :: Repository ,) -> CargoResult < git2 :: Commit < 'a > > { let head_commit = match args . get_one :: < String > ("head-rev") { Some (sha) => { let head_obj = repo . revparse_single (sha) ? ; head_obj . peel_to_commit () ? } None => { let head_ref = repo . head () ? ; head_ref . peel_to_commit () ? } } ; Ok (head_commit) }
};
}
