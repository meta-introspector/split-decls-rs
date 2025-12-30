// Generated macro for init_with_env (function)
macro_rules! Depcrate_androidinit_with_env {
() => {
// Module: crate::android
// Provides: {"init_with_env"}
// Dependencies: {}
# [doc = " Initialize given a typical Android NDK [`JNIEnv`] and [`JObject`] context."] # [doc = ""] # [doc = " This method will setup and store an environment locally. This is useful if nothing else in your"] # [doc = " application needs to access the Android runtime."] pub fn init_with_env (env : & mut JNIEnv , context : JObject) -> Result < () , JNIError > { GLOBAL . get_or_try_init (| | -> Result < _ , JNIError > { let loader = env . call_method (& context , "getClassLoader" , "()Ljava/lang/ClassLoader;" , & []) ? ; Ok (Global :: Internal { java_vm : env . get_java_vm () ? , context : env . new_global_ref (context) ? , loader : env . new_global_ref (JObject :: try_from (loader) ?) ? , }) }) ? ; Ok (()) }
};
}
