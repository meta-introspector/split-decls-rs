// Generated macro for impl_441 (impl)
macro_rules! Depcrate_foldimpl_441 {
() => {
// Module: crate::fold
// Provides: {"impl_441"}
// Dependencies: {}
impl < I : Interner , T : TypeFoldable < I > , Ix : Idx > TypeFoldable < I > for IndexVec < Ix , T > { fn try_fold_with < F : FallibleTypeFolder < I > > (self , folder : & mut F) -> Result < Self , F :: Error > { self . raw . try_fold_with (folder) . map (IndexVec :: from_raw) } fn fold_with < F : TypeFolder < I > > (self , folder : & mut F) -> Self { IndexVec :: from_raw (self . raw . fold_with (folder)) } }
};
}
