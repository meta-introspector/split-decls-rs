// Generated macro for CompilerProxy (trait)
macro_rules! Depcrate_compiler_compilerCompilerProxy {
() => {
// Module: crate::compiler::compiler
// Provides: {"CompilerProxy"}
// Dependencies: {}
pub trait CompilerProxy < T > : Send + Sync + 'static where T : CommandCreatorSync + Sized , { # [doc = " Maps the executable to be used in `cwd` to the true, proxied compiler."] # [doc = ""] # [doc = " Returns the absolute path to the true compiler and the timestamp of"] # [doc = " timestamp of the true compiler. Iff the resolution fails,"] # [doc = " the returned future resolves to an error with more information."] fn resolve_proxied_executable (& self , creator : T , cwd : PathBuf , env_vars : & [(OsString , OsString)] ,) -> Pin < Box < dyn Future < Output = Result < (PathBuf , FileTime) > > + Send + 'static > > ; # [doc = " Create a clone of `Self` and puts it in a `Box`"] fn box_clone (& self) -> Box < dyn CompilerProxy < T > > ; }
};
}
