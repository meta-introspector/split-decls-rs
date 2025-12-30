// Generated macro for impl_48 (impl)
macro_rules! Depcrate_errorimpl_48 {
() => {
// Module: crate::error
// Provides: {"impl_48"}
// Dependencies: {}
impl < T > IoResultExt < T > for Result < T , io :: Error > { fn with_err_path < F , P > (self , path : F) -> Self where F : FnOnce () -> P , P : Into < PathBuf > , { self . map_err (| e | { io :: Error :: new (e . kind () , PathError { path : path () . into () , err : e , } ,) }) } }
};
}
