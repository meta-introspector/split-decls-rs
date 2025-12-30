// Generated macro for impl_436 (impl)
macro_rules! Depcrate_foldimpl_436 {
() => {
// Module: crate::fold
// Provides: {"impl_436"}
// Dependencies: {}
impl < I : Interner , T : TypeFoldable < I > > TypeFoldable < I > for Arc < T > { fn try_fold_with < F : FallibleTypeFolder < I > > (self , folder : & mut F) -> Result < Self , F :: Error > { fold_arc (self , | t | t . try_fold_with (folder)) } fn fold_with < F : TypeFolder < I > > (self , folder : & mut F) -> Self { match fold_arc :: < T , Infallible > (self , | t | Ok (t . fold_with (folder))) { Ok (t) => t , } } }
};
}
