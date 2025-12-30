// Generated macro for macro_32 (macro)
macro_rules! Depcrate_listablemacro_32 {
() => {
// Module: crate::listable
// Provides: {"macro_32"}
// Dependencies: {}
deref ! { & T , & mut T , # [cfg (feature = "alloc")] alloc :: boxed :: Box < T >, # [cfg (feature = "alloc")] alloc :: rc :: Rc < T >, # [cfg (not (valuable_no_atomic_cas))] # [cfg (feature = "alloc")] alloc :: sync :: Arc < T >, }
};
}
