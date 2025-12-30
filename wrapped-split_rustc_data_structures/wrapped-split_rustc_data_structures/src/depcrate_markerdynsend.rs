// Generated macro for DynSend (trait)
macro_rules! Depcrate_markerDynSend {
() => {
// Module: crate::marker
// Provides: {"DynSend"}
// Dependencies: {}
# [diagnostic :: on_unimplemented (message = "`{Self}` doesn't implement `DynSend`. \
            Add it to `rustc_data_structures::marker` or use `IntoDynSyncSend` if it's already `Send`")] pub unsafe auto trait DynSend { }
};
}
