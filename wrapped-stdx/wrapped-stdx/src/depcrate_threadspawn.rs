// Generated macro for spawn (function)
macro_rules! Depcrate_threadspawn {
() => {
// Module: crate::thread
// Provides: {"spawn"}
// Dependencies: {}
# [doc = " # Panics"] # [doc = ""] # [doc = " Panics if failed to spawn the thread."] pub fn spawn < F , T > (intent : ThreadIntent , name : String , f : F) -> JoinHandle < T > where F : (FnOnce () -> T) + Send + 'static , T : Send + 'static , { Builder :: new (intent , name) . spawn (f) . expect ("failed to spawn thread") }
};
}
