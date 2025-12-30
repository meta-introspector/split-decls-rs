// Generated macro for IdentityMap (struct)
macro_rules! Depcrate_tracked_structIdentityMap {
() => {
// Module: crate::tracked_struct
// Provides: {"IdentityMap"}
// Dependencies: {}
# [doc = " A map from tracked struct [`Identity`] to their final [`Id`]."] # [derive (Default , Debug)] pub (crate) struct IdentityMap { table : hashbrown :: HashTable < TrackedEntry > , }
};
}
