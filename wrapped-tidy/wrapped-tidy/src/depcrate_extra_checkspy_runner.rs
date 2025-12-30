// Generated macro for py_runner (function)
macro_rules! Depcrate_extra_checkspy_runner {
() => {
// Module: crate::extra_checks
// Provides: {"py_runner"}
// Dependencies: {}
# [doc = " Run a python command with given arguments. `py_path` should be a virtualenv."] # [doc = ""] # [doc = " Captures `stdout` to a string if provided, otherwise prints the output."] fn py_runner (py_path : & Path , as_module : bool , stdout : Option < & mut String > , bin : & 'static str , args : & [& OsStr] ,) -> Result < () , Error > { let mut cmd = Command :: new (py_path) ; if as_module { cmd . arg ("-m") . arg (bin) . args (args) ; } else { let bin_path = py_path . with_file_name (bin) ; cmd . arg (bin_path) . args (args) ; } let status = if let Some (stdout) = stdout { let output = cmd . output () ? ; if let Ok (s) = std :: str :: from_utf8 (& output . stdout) { stdout . push_str (s) ; } output . status } else { cmd . status () ? } ; if status . success () { Ok (()) } else { Err (Error :: FailedCheck (bin)) } }
};
}
