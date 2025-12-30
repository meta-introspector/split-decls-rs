// Generated macro for init_hosted (function)
macro_rules! Depcrate_androidinit_hosted {
() => {
// Module: crate::android
// Provides: {"init_hosted"}
// Dependencies: {}
# [doc = " *Deprecated*: This is the original method name for [`init_with_env`] and is functionally"] # [doc = " identical."] pub fn init_hosted (env : & mut JNIEnv , context : JObject) -> Result < () , JNIError > { init_with_env (env , context) }
};
}
