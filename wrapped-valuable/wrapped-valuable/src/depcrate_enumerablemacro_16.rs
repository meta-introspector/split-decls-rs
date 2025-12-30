// Generated macro for macro_16 (macro)
macro_rules! Depcrate_enumerablemacro_16 {
() => {
// Module: crate::enumerable
// Provides: {"macro_16"}
// Dependencies: {}
deref ! { & T , & mut T , # [cfg (feature = "alloc")] alloc :: boxed :: Box < T >, # [cfg (feature = "alloc")] alloc :: rc :: Rc < T >, # [cfg (not (valuable_no_atomic_cas))] # [cfg (feature = "alloc")] alloc :: sync :: Arc < T >, }
};
}
