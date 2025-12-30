// Generated macro for merge_args (function)
macro_rules! Depcrate_extra_checksmerge_args {
() => {
// Module: crate::extra_checks
// Provides: {"merge_args"}
// Dependencies: {}
# [doc = " Helper to create `cfg1 cfg2 -- file1 file2` output"] fn merge_args < 'a > (cfg_args : & [& 'a OsStr] , file_args : & [& 'a OsStr]) -> Vec < & 'a OsStr > { let mut args = cfg_args . to_owned () ; args . push ("--" . as_ref ()) ; args . extend (file_args) ; args }
};
}
