// Generated macro for macro_34 (macro)
macro_rules! Depcrate_listablemacro_34 {
() => {
// Module: crate::listable
// Provides: {"macro_34"}
// Dependencies: {}
slice ! { (T : Valuable) &'_ [T] , # [cfg (feature = "alloc")] (T : Valuable) alloc :: boxed :: Box < [T] >, # [cfg (feature = "alloc")] (T : Valuable) alloc :: rc :: Rc < [T] >, # [cfg (not (valuable_no_atomic_cas))] # [cfg (feature = "alloc")] (T : Valuable) alloc :: sync :: Arc < [T] >, (T : Valuable , const N : usize) [T ; N] , # [cfg (feature = "alloc")] (T : Valuable) alloc :: vec :: Vec < T >, }
};
}
