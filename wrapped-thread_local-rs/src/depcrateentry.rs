// Generated macro for Entry (struct)
macro_rules! DepcrateEntry {
() => {
// Module: crate
// Provides: {"Entry"}
// Dependencies: {}
struct Entry < T > { present : AtomicBool , value : UnsafeCell < MaybeUninit < T > > , }
};
}
