// Generated macro for impl_331 (impl)
macro_rules! Depcrate_binderimpl_331 {
() => {
// Module: crate::binder
// Provides: {"impl_331"}
// Dependencies: {}
impl < I : Interner , T : TypeFoldable < I > > TypeFoldable < I > for Binder < I , T > { fn try_fold_with < F : FallibleTypeFolder < I > > (self , folder : & mut F) -> Result < Self , F :: Error > { folder . try_fold_binder (self) } fn fold_with < F : TypeFolder < I > > (self , folder : & mut F) -> Self { folder . fold_binder (self) } }
};
}
