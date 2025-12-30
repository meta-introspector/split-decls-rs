// Generated macro for abort (function)
macro_rules! Depcrateabort {
() => {
// Module: crate
// Provides: {"abort"}
// Dependencies: {}
# [cfg (not (feature = "std"))] # [cold] fn abort () -> ! { struct PanicOnDrop ; impl Drop for PanicOnDrop { fn drop (& mut self) { panic ! () } } let _double_panicer = PanicOnDrop ; panic ! () ; }
};
}
