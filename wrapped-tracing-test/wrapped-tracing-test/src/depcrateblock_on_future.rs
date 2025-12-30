// Generated macro for block_on_future (function)
macro_rules! Depcrateblock_on_future {
() => {
// Module: crate
// Provides: {"block_on_future"}
// Dependencies: {}
pub fn block_on_future < F > (future : F) -> F :: Output where F : std :: future :: Future + Send + 'static , F :: Output : Send + 'static , { tokio_test :: block_on (future) }
};
}
