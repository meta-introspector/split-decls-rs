// Generated macro for impl_124 (impl)
macro_rules! Depcrate_errorimpl_124 {
() => {
// Module: crate::error
// Provides: {"impl_124"}
// Dependencies: {}
# [cfg (feature = "std")] impl < I , C , E : std :: error :: Error + Send + Sync + 'static > FromExternalError < I , E > for TreeError < I , C > where I : Clone , { fn from_external_error (input : & I , e : E) -> Self { TreeError :: Base (TreeErrorBase { input : input . clone () , cause : Some (Box :: new (e)) , }) } }
};
}
