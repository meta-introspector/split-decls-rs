// Generated macro for impl_314 (impl)
macro_rules! Depcrate_move_pathsimpl_314 {
() => {
// Module: crate::move_paths
// Provides: {"impl_314"}
// Dependencies: {}
impl < T > LocationMap < T > where T : Default + Clone , { fn new (body : & Body < '_ >) -> Self { LocationMap { map : body . basic_blocks . iter () . map (| block | vec ! [T :: default () ; block . statements . len () + 1]) . collect () , } } }
};
}
