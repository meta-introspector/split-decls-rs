// Generated macro for create_stream (function)
macro_rules! Depcratecreate_stream {
() => {
// Module: crate
// Provides: {"create_stream"}
// Dependencies: {}
pub fn create_stream () -> impl Stream < Item = u8 > { reexporter :: stream ! { for x in 0 .. 10_u8 { yield x ; } } }
};
}
