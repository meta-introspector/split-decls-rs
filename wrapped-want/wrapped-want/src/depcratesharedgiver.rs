// Generated macro for SharedGiver (struct)
macro_rules! DepcrateSharedGiver {
() => {
// Module: crate
// Provides: {"SharedGiver"}
// Dependencies: {}
# [doc = " A cloneable `Giver`."] # [doc = ""] # [doc = " It differs from `Giver` in that you cannot poll for `want`. It's only"] # [doc = " usable as a cancellation watcher."] # [derive (Clone)] pub struct SharedGiver { inner : Arc < Inner > , }
};
}
