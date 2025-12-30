// Generated macro for macro_82 (macro)
macro_rules! Depcrate_structablemacro_82 {
() => {
// Module: crate::structable
// Provides: {"macro_82"}
// Dependencies: {}
deref ! { & T , & mut T , # [cfg (feature = "alloc")] alloc :: boxed :: Box < T >, # [cfg (feature = "alloc")] alloc :: rc :: Rc < T >, # [cfg (not (valuable_no_atomic_cas))] # [cfg (feature = "alloc")] alloc :: sync :: Arc < T >, }
};
}
