// Generated macro for with (function)
macro_rules! Depcrate_routewith {
() => {
// Module: crate::route
// Provides: {"with"}
// Dependencies: {}
pub (crate) fn with < F , R > (func : F) -> R where F : FnOnce (& mut Route) -> R , { ROUTE . with (move | route | func (& mut * route . borrow_mut ())) }
};
}
