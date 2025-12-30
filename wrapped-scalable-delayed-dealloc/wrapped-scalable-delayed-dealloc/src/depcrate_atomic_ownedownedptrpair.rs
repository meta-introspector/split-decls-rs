// Generated macro for OwnedPtrPair (type)
macro_rules! Depcrate_atomic_ownedOwnedPtrPair {
() => {
// Module: crate::atomic_owned
// Provides: {"OwnedPtrPair"}
// Dependencies: {}
# [doc = " A pair of [`Owned`] and [`Ptr`] of the same type."] pub type OwnedPtrPair < 'g , T > = (Option < Owned < T > > , Ptr < 'g , T >) ;
};
}
