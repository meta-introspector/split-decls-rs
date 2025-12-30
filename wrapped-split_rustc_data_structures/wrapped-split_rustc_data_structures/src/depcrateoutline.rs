// Generated macro for outline (function)
macro_rules! Depcrateoutline {
() => {
// Module: crate
// Provides: {"outline"}
// Dependencies: {}
# [doc = " This calls the passed function while ensuring it won't be inlined into the caller."] # [inline (never)] # [cold] pub fn outline < F : FnOnce () -> R , R > (f : F) -> R { f () }
};
}
