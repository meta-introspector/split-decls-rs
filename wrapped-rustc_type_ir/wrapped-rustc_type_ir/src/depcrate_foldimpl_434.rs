// Generated macro for impl_434 (impl)
macro_rules! Depcrate_foldimpl_434 {
() => {
// Module: crate::fold
// Provides: {"impl_434"}
// Dependencies: {}
impl < I : Interner , T : TypeFoldable < I > , E : TypeFoldable < I > > TypeFoldable < I > for Result < T , E > { fn try_fold_with < F : FallibleTypeFolder < I > > (self , folder : & mut F) -> Result < Self , F :: Error > { Ok (match self { Ok (v) => Ok (v . try_fold_with (folder) ?) , Err (e) => Err (e . try_fold_with (folder) ?) , }) } fn fold_with < F : TypeFolder < I > > (self , folder : & mut F) -> Self { match self { Ok (v) => Ok (v . fold_with (folder)) , Err (e) => Err (e . fold_with (folder)) , } } }
};
}
