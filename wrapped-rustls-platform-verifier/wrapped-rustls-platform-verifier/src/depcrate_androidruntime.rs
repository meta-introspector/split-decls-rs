// Generated macro for Runtime (trait)
macro_rules! Depcrate_androidRuntime {
() => {
// Module: crate::android
// Provides: {"Runtime"}
// Dependencies: {}
# [doc = " A layer to access the Android runtime which is hosting the current"] # [doc = " application process."] # [doc = ""] # [doc = " Generally this trait should be implemented in your Rust app component's FFI"] # [doc = " initialization layer."] pub trait Runtime : Send + Sync { # [doc = " Returns a handle to the current process' JVM."] fn java_vm (& self) -> & JavaVM ; # [doc = " Returns a reference to the current app's [Context]."] # [doc = ""] # [doc = " [Context]: <https://developer.android.com/reference/android/content/Context>"] fn context (& self) -> & GlobalRef ; # [doc = " Returns a reference to the class returned by the current JVM's `getClassLoader` call."] fn class_loader (& self) -> & GlobalRef ; }
};
}
