// Generated macro for Iter (struct)
macro_rules! DepcrateIter {
() => {
// Module: crate
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " Iterator over the contents of a `ThreadLocal`."] # [derive (Debug)] pub struct Iter < 'a , T : Send + Sync > { thread_local : & 'a ThreadLocal < T > , raw : RawIter , }
};
}
