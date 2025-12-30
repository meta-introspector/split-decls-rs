// Generated macro for with_context (function)
macro_rules! Depcrate_androidwith_context {
() => {
// Module: crate::android
// Provides: {"with_context"}
// Dependencies: {}
# [doc = " Borrow the Android application context and execute the closure"] # [doc = " `with_context, ensuring locals are properly freed and exceptions"] # [doc = " are cleared."] pub (super) fn with_context < F , T : 'static > (f : F) -> Result < T , Error > where F : FnOnce (& mut LocalContext , & mut JNIEnv) -> Result < T , Error > , { let (global_context , mut binding_env) = global () . context () ? ; let ctx_env = unsafe { binding_env . unsafe_clone () } ; binding_env . with_local_frame (16 , | env | { let mut ctx_env = ctx_env ; let mut context = LocalContext { env : & mut ctx_env , context : global_context . context , loader : global_context . loader , } ; f (& mut context , env) }) }
};
}
