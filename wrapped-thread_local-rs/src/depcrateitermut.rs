// Generated macro for IterMut (struct)
macro_rules! DepcrateIterMut {
() => {
// Module: crate
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " Mutable iterator over the contents of a `ThreadLocal`."] pub struct IterMut < 'a , T : Send > { thread_local : & 'a mut ThreadLocal < T > , raw : RawIter , }
};
}
