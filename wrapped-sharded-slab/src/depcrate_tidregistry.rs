// Generated macro for Registry (struct)
macro_rules! Depcrate_tidRegistry {
() => {
// Module: crate::tid
// Provides: {"Registry"}
// Dependencies: {}
struct Registry { next : AtomicUsize , free : Mutex < VecDeque < usize > > , }
};
}
