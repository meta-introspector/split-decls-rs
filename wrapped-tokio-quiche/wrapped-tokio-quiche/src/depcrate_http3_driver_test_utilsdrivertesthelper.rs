// Generated macro for DriverTestHelper (struct)
macro_rules! Depcrate_http3_driver_test_utilsDriverTestHelper {
() => {
// Module: crate::http3::driver::test_utils
// Provides: {"DriverTestHelper"}
// Dependencies: {}
# [doc = " Similar to `quiche::test_utils::Pipe`, a wrapper with helper functions"] # [doc = " for a client and server endpoint. One endpoint is driven by an H3Driver"] # [doc = " to allow testing the H3Driver logic. The other endpoint (the peer) is"] # [doc = " driven directly by an `quiche::h3::Connection`."] pub struct DriverTestHelper < H : DriverHooks + GetConnectionForHook > { pub pipe : quiche :: test_utils :: Pipe , pub driver : H3Driver < H > , pub controller : H3Controller < H > , # [doc = " Our peer, not using a driver, just the h3::Connection directly"] pub peer : h3 :: Connection , }
};
}
