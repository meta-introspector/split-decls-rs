// Generated macro for threaded_read (function)
macro_rules! Depcrate_cmdthreaded_read {
() => {
// Module: crate::cmd
// Provides: {"threaded_read"}
// Dependencies: {}
fn threaded_read < R > (mut input : R) -> Stream where R : std :: io :: Read + Send + 'static , { std :: thread :: spawn (move | | { let mut ret = Vec :: new () ; input . read_to_end (& mut ret) . map (| _ | ret) }) }
};
}
