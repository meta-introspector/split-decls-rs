// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl Config { fn from_env () -> Self { let buffer_size = env :: var ("BUFFER_SIZE") . ok () . and_then (| v | v . parse () . ok ()) . unwrap_or (BUFFER_SIZE) ; let buffer_count = env :: var ("BUFFER_COUNT") . ok () . and_then (| v | v . parse () . ok ()) . unwrap_or (BUFFER_COUNT) ; Self { buffer_size , buffer_count , } } }
};
}
