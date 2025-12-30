// Generated macro for error (function)
macro_rules! Depcrate_layouterror {
() => {
// Module: crate::layout
// Provides: {"error"}
// Dependencies: {}
fn error < 'tcx > (cx : & LayoutCx < 'tcx > , err : LayoutError < 'tcx >) -> & 'tcx LayoutError < 'tcx > { cx . tcx () . arena . alloc (err) }
};
}
