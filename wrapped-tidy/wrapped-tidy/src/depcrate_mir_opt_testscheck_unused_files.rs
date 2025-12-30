// Generated macro for check_unused_files (function)
macro_rules! Depcrate_mir_opt_testscheck_unused_files {
() => {
// Module: crate::mir_opt_tests
// Provides: {"check_unused_files"}
// Dependencies: {}
fn check_unused_files (path : & Path , bless : bool , bad : & mut bool) { let mut rs_files = Vec :: < PathBuf > :: new () ; let mut output_files = HashSet :: < PathBuf > :: new () ; walk_no_read (& [& path . join ("mir-opt")] , | path , _is_dir | path . file_name () == Some ("README.md" . as_ref ()) , & mut | file | { let filepath = file . path () ; if filepath . extension () == Some ("rs" . as_ref ()) { rs_files . push (filepath . to_owned ()) ; } else { output_files . insert (filepath . to_owned ()) ; } } ,) ; for file in rs_files { for bw in [32 , 64] { for ps in [PanicStrategy :: Unwind , PanicStrategy :: Abort] { let mir_opt_test = miropt_test_tools :: files_for_miropt_test (& file , bw , ps) ; for output_file in mir_opt_test . files { output_files . remove (& output_file . expected_file) ; } } } } for extra in output_files { if ! bless { tidy_error ! (bad , "the following output file is not associated with any mir-opt test, you can remove it: {}" , extra . display ()) ; } else { let _ = std :: fs :: remove_file (extra) ; } } }
};
}
