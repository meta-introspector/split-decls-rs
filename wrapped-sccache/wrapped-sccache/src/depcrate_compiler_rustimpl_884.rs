// Generated macro for impl_884 (impl)
macro_rules! Depcrate_compiler_rustimpl_884 {
() => {
// Module: crate::compiler::rust
// Provides: {"impl_884"}
// Dependencies: {}
impl FromArg for ArgCrateTypes { fn process (arg : OsString) -> ArgParseResult < Self > { let arg = String :: process (arg) ? ; let mut crate_types = ArgCrateTypes { rlib : false , staticlib : false , others : HashSet :: new () , } ; for ty in arg . split (',') { match ty { "lib" | "rlib" => crate_types . rlib = true , "staticlib" => crate_types . staticlib = true , other => { crate_types . others . insert (other . to_owned ()) ; } } } Ok (crate_types) } }
};
}
