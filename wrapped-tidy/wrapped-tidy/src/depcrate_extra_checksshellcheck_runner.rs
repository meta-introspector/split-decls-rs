// Generated macro for shellcheck_runner (function)
macro_rules! Depcrate_extra_checksshellcheck_runner {
() => {
// Module: crate::extra_checks
// Provides: {"shellcheck_runner"}
// Dependencies: {}
# [doc = " Check that shellcheck is installed then run it at the given path"] fn shellcheck_runner (args : & [& OsStr]) -> Result < () , Error > { match Command :: new ("shellcheck") . arg ("--version") . status () { Ok (_) => () , Err (e) if e . kind () == io :: ErrorKind :: NotFound => { return Err (Error :: MissingReq ("shellcheck" , "shell file checks" , Some ("see <https://github.com/koalaman/shellcheck#installing> \
                    for installation instructions" . to_owned () ,) ,)) ; } Err (e) => return Err (e . into ()) , } let status = Command :: new ("shellcheck") . args (args) . status () ? ; if status . success () { Ok (()) } else { Err (Error :: FailedCheck ("shellcheck")) } }
};
}
