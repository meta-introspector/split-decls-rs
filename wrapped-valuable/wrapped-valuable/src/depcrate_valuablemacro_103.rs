// Generated macro for macro_103 (macro)
macro_rules! Depcrate_valuablemacro_103 {
() => {
// Module: crate::valuable
// Provides: {"macro_103"}
// Dependencies: {}
deref ! { & T , & mut T , # [cfg (feature = "alloc")] alloc :: boxed :: Box < T >, # [cfg (feature = "alloc")] alloc :: rc :: Rc < T >, # [cfg (not (valuable_no_atomic_cas))] # [cfg (feature = "alloc")] alloc :: sync :: Arc < T >, }
};
}
