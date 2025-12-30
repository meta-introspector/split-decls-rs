// Generated macro for macro_46 (macro)
macro_rules! Depcrate_mappablemacro_46 {
() => {
// Module: crate::mappable
// Provides: {"macro_46"}
// Dependencies: {}
deref ! { & T , & mut T , # [cfg (feature = "alloc")] alloc :: boxed :: Box < T >, # [cfg (feature = "alloc")] alloc :: rc :: Rc < T >, # [cfg (not (valuable_no_atomic_cas))] # [cfg (feature = "alloc")] alloc :: sync :: Arc < T >, }
};
}
