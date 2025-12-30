// Generated macro for impl_22 (impl)
macro_rules! Depcrate_schemaimpl_22 {
() => {
// Module: crate::schema
// Provides: {"impl_22"}
// Dependencies: {}
impl Args { fn new () -> Self { Self :: Split (Default :: default ()) } fn as_slice (& self) -> & [String] { match self { Self :: Joined (j) => j . inner . as_slice () , Self :: Split (v) => v . as_slice () , } } fn into_vec (self) -> Vec < String > { match self { Self :: Joined (j) => j . inner , Self :: Split (v) => v , } } }
};
}
