// Generated macro for set (function)
macro_rules! Depcrate_routeset {
() => {
// Module: crate::route
// Provides: {"set"}
// Dependencies: {}
pub (crate) fn set < F , U > (r : & RefCell < Route > , func : F) -> U where F : FnOnce () -> U , { ROUTE . set (r , func) }
};
}
