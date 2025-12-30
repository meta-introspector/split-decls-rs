// Generated macro for impl_118 (impl)
macro_rules! Depcrate_errorimpl_118 {
() => {
// Module: crate::error
// Provides: {"impl_118"}
// Dependencies: {}
impl From < Error > for ControlFlow < Error , Error > { fn from (value : Error) -> Self { match value { err if err . is_fatal () => Self :: Break (err) , err => Self :: Continue (err) , } } }
};
}
