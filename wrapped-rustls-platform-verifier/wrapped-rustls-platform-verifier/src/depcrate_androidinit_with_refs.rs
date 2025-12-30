// Generated macro for init_with_refs (function)
macro_rules! Depcrate_androidinit_with_refs {
() => {
// Module: crate::android
// Provides: {"init_with_refs"}
// Dependencies: {}
# [doc = " Initialize with references to the JVM, context, and class loader."] # [doc = ""] # [doc = " This is useful when you're already interacting with `jni-rs` wrapped objects and want to use"] # [doc = " global references to objects for efficiency."] # [doc = ""] # [doc = " This function will never panic."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " pub fn android_init(raw_env: *mut c_void, raw_context: *mut c_void) -> Result<(), jni::errors::Error> {"] # [doc = "     let mut env = unsafe { jni::JNIEnv::from_raw(raw_env as *mut jni::sys::JNIEnv).unwrap() };"] # [doc = "     let context = unsafe { JObject::from_raw(raw_context as jni::sys::jobject) };"] # [doc = "     let loader = env.call_method(&context, \"getClassLoader\", \"()Ljava/lang/ClassLoader;\", &[])?;"] # [doc = ""] # [doc = "     rustls_platform_verifier::android::init_with_refs("] # [doc = "         env.get_java_vm()?,"] # [doc = "         env.new_global_ref(context)?,"] # [doc = "         env.new_global_ref(JObject::try_from(loader)?)?,"] # [doc = "     );"] # [doc = " }"] # [doc = " ```"] pub fn init_with_refs (java_vm : JavaVM , context : GlobalRef , loader : GlobalRef) { GLOBAL . get_or_init (| | Global :: Internal { java_vm , context , loader , }) ; }
};
}
