// Generated macro for impl_431 (impl)
macro_rules! Depcrate_foldimpl_431 {
() => {
// Module: crate::fold
// Provides: {"impl_431"}
// Dependencies: {}
impl < I : Interner , T : TypeFoldable < I > , U : TypeFoldable < I > > TypeFoldable < I > for (T , U) { fn try_fold_with < F : FallibleTypeFolder < I > > (self , folder : & mut F) -> Result < (T , U) , F :: Error > { Ok ((self . 0 . try_fold_with (folder) ? , self . 1 . try_fold_with (folder) ?)) } fn fold_with < F : TypeFolder < I > > (self , folder : & mut F) -> Self { (self . 0 . fold_with (folder) , self . 1 . fold_with (folder)) } }
};
}
