// Generated macro for impl_80 (impl)
macro_rules! Depcrate_valueimpl_80 {
() => {
// Module: crate::value
// Provides: {"impl_80"}
// Dependencies: {}
impl < V : Into < Self > > From < Vec < V > > for Value { fn from (val : Vec < V >) -> Self { Self :: Array (val . into_iter () . map (| v | v . into ()) . collect ()) } }
};
}
