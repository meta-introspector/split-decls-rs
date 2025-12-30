// Generated macro for local_serial_core_with_return (function)
macro_rules! Depcrate_serial_code_locklocal_serial_core_with_return {
() => {
// Module: crate::serial_code_lock
// Provides: {"local_serial_core_with_return"}
// Dependencies: {}
# [doc (hidden)] pub fn local_serial_core_with_return < R , E > (names : Vec < & str > , _path : Option < String > , function : fn () -> Result < R , E > ,) -> Result < R , E > { core_internal ! (names) ; function () }
};
}
