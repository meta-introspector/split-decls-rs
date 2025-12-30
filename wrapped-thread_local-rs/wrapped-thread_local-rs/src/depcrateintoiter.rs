// Generated macro for IntoIter (struct)
macro_rules! DepcrateIntoIter {
() => {
// Module: crate
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " An iterator that moves out of a `ThreadLocal`."] # [derive (Debug)] pub struct IntoIter < T : Send > { thread_local : ThreadLocal < T > , raw : RawIter , }
};
}
