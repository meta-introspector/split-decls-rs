// Generated macro for impl_437 (impl)
macro_rules! Depcrate_foldimpl_437 {
() => {
// Module: crate::fold
// Provides: {"impl_437"}
// Dependencies: {}
impl < I : Interner , T : TypeFoldable < I > > TypeFoldable < I > for Box < T > { fn try_fold_with < F : FallibleTypeFolder < I > > (mut self , folder : & mut F) -> Result < Self , F :: Error > { * self = (* self) . try_fold_with (folder) ? ; Ok (self) } fn fold_with < F : TypeFolder < I > > (mut self , folder : & mut F) -> Self { * self = (* self) . fold_with (folder) ; self } }
};
}
