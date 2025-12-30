// Generated macro for is_zeroed (function)
macro_rules! Depcrate_pointeris_zeroed {
() => {
// Module: crate::pointer
// Provides: {"is_zeroed"}
// Dependencies: {}
# [doc = " Checks if the referent is zeroed."] pub (crate) fn is_zeroed < T , I > (ptr : Ptr < '_ , T , I >) -> bool where T : crate :: Immutable + crate :: KnownLayout , I : invariant :: Invariants < Validity = invariant :: Initialized > , I :: Aliasing : invariant :: Reference , { ptr . as_bytes :: < BecauseImmutable > () . as_ref () . iter () . all (| & byte | byte == 0) }
};
}
