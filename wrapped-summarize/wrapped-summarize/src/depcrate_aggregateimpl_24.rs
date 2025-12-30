// Generated macro for impl_24 (impl)
macro_rules! Depcrate_aggregateimpl_24 {
() => {
// Module: crate::aggregate
// Provides: {"impl_24"}
// Dependencies: {}
impl < 'a > From < WithParent < Event < 'a > > > for WithParent < EventDescription < 'a > > { fn from (e : WithParent < Event < 'a > >) -> Self { WithParent { this : e . this . into () , parent : e . parent . map (| e | e . into ()) , } } }
};
}
