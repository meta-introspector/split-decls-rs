// Generated macro for impl_66 (impl)
macro_rules! Depcrate_distimpl_66 {
() => {
// Module: crate::dist
// Provides: {"impl_66"}
// Dependencies: {}
impl Target { fn get (project_root : & Path , sh : & Shell) -> Self { let name = detect_target (sh) ; let (name , libc_suffix) = match name . split_once ('.') { Some ((l , r)) => (l . to_owned () , Some (r . to_owned ())) , None => (name , None) , } ; let out_path = project_root . join ("target") . join (& name) . join ("release") ; let (exe_suffix , symbols_path) = if name . contains ("-windows-") { (".exe" . into () , Some (out_path . join ("rust_analyzer.pdb"))) } else { (String :: new () , None) } ; let server_path = out_path . join (format ! ("rust-analyzer{exe_suffix}")) ; let artifact_name = format ! ("rust-analyzer-{name}{exe_suffix}") ; Self { name , libc_suffix , server_path , symbols_path , artifact_name } } fn is_linux (& self) -> bool { self . name . contains ("-linux-") } }
};
}
