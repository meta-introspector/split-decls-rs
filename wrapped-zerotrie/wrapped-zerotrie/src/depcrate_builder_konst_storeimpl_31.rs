// Generated macro for impl_31 (impl)
macro_rules! Depcrate_builder_konst_storeimpl_31 {
() => {
// Module: crate::builder::konst::store
// Provides: {"impl_31"}
// Dependencies: {}
impl < const N : usize , T : Default > Default for ConstArrayBuilder < N , T > { fn default () -> Self { Self :: new_empty ([() ; N] . map (| _ | Default :: default ()) , 0) } }
};
}
