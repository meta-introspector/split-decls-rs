// Generated macro for create_try_stream (function)
macro_rules! Depcratecreate_try_stream {
() => {
// Module: crate
// Provides: {"create_try_stream"}
// Dependencies: {}
pub fn create_try_stream () -> impl Stream < Item = Result < u8 , u8 > > { reexporter :: try_stream ! { for x in 0 .. 10_u8 { yield x ; } } }
};
}
