// Generated macro for invoke (function)
macro_rules! Depcrateinvoke {
() => {
// Module: crate
// Provides: {"invoke"}
// Dependencies: {}
# [doc (hidden)] pub fn invoke < T , R , F : FnOnce (T) -> R > (f : F , t : T) -> R { f (t) }
};
}
