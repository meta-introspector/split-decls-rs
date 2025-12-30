// Generated macro for install_requirements (function)
macro_rules! Depcrate_extra_checksinstall_requirements {
() => {
// Module: crate::extra_checks
// Provides: {"install_requirements"}
// Dependencies: {}
fn install_requirements (py_path : & Path , src_reqs_path : & Path , dst_reqs_path : & Path ,) -> Result < () , Error > { let stat = Command :: new (py_path) . args (["-m" , "pip" , "install" , "--upgrade" , "pip"]) . status () . expect ("failed to launch pip") ; if ! stat . success () { return Err (Error :: Generic (format ! ("pip install failed with status {stat}"))) ; } let stat = Command :: new (py_path) . args (["-m" , "pip" , "install" , "--quiet" , "--require-hashes" , "-r"]) . arg (src_reqs_path) . status () ? ; if ! stat . success () { return Err (Error :: Generic (format ! ("failed to install requirements at {}" , src_reqs_path . display ()))) ; } fs :: copy (src_reqs_path , dst_reqs_path) ? ; assert_eq ! (fs :: read_to_string (src_reqs_path) . unwrap () , fs :: read_to_string (dst_reqs_path) . unwrap ()) ; Ok (()) }
};
}
