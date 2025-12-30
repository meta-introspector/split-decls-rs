// Generated macro for macro_90 (macro)
macro_rules! Depcrate_tuplablemacro_90 {
() => {
// Module: crate::tuplable
// Provides: {"macro_90"}
// Dependencies: {}
deref ! { & T , & mut T , # [cfg (feature = "alloc")] alloc :: boxed :: Box < T >, # [cfg (feature = "alloc")] alloc :: rc :: Rc < T >, # [cfg (not (valuable_no_atomic_cas))] # [cfg (feature = "alloc")] alloc :: sync :: Arc < T >, }
};
}
