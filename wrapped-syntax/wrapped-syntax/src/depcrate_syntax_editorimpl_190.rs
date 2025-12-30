// Generated macro for impl_190 (impl)
macro_rules! Depcrate_syntax_editorimpl_190 {
() => {
// Module: crate::syntax_editor
// Provides: {"impl_190"}
// Dependencies: {}
impl Default for SyntaxAnnotation { fn default () -> Self { static COUNTER : AtomicU32 = AtomicU32 :: new (1) ; let id = COUNTER . fetch_add (1 , Ordering :: Relaxed) ; Self (NonZeroU32 :: new (id) . expect ("syntax annotation id overflow")) } }
};
}
