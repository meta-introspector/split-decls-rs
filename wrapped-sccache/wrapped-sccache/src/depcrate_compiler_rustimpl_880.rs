// Generated macro for impl_880 (impl)
macro_rules! Depcrate_compiler_rustimpl_880 {
() => {
// Module: crate::compiler::rust
// Provides: {"impl_880"}
// Dependencies: {}
impl < T > CompilerProxy < T > for RustupProxy where T : CommandCreatorSync , { fn resolve_proxied_executable (& self , mut creator : T , cwd : PathBuf , env : & [(OsString , OsString)] ,) -> Pin < Box < dyn Future < Output = Result < (PathBuf , FileTime) > > + Send > > { let mut child = creator . new_command_sync (& self . proxy_executable) ; child . current_dir (& cwd) . env_clear () . envs (env . to_vec ()) . args (& ["which" , "rustc"]) ; Box :: pin (async move { let output = run_input_output (child , None) . await . context ("Failed to execute rustup which rustc") ? ; let stdout = String :: from_utf8 (output . stdout) . context ("Failed to parse output of rustup which rustc") ? ; let proxied_compiler = PathBuf :: from (stdout . trim ()) ; trace ! ("proxy: rustup which rustc produced: {:?}" , & proxied_compiler) ; let attr = fs :: metadata (proxied_compiler . as_path ()) . context ("Failed to obtain metadata of the resolved, true rustc") ? ; if attr . is_file () { Ok (FileTime :: from_last_modification_time (& attr)) } else { Err (anyhow ! ("proxy: rustup resolved compiler is not of type file")) } . map (move | filetime | (proxied_compiler , filetime)) }) } fn box_clone (& self) -> Box < dyn CompilerProxy < T > > { Box :: new ((* self) . clone ()) } }
};
}
