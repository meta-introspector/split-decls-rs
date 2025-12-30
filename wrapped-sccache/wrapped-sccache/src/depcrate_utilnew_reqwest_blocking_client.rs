// Generated macro for new_reqwest_blocking_client (function)
macro_rules! Depcrate_utilnew_reqwest_blocking_client {
() => {
// Module: crate::util
// Provides: {"new_reqwest_blocking_client"}
// Dependencies: {}
# [doc = " Disable connection pool to avoid broken connection between runtime"] # [doc = ""] # [doc = " # TODO"] # [doc = ""] # [doc = " We should refactor sccache current model to make sure that we only have"] # [doc = " one tokio runtime and keep reqwest alive inside it."] # [doc = ""] # [doc = " ---"] # [doc = ""] # [doc = " More details could be found at https://github.com/mozilla/sccache/pull/1563"] # [cfg (any (feature = "dist-server" , feature = "dist-client"))] pub fn new_reqwest_blocking_client () -> reqwest :: blocking :: Client { reqwest :: blocking :: Client :: builder () . pool_max_idle_per_host (0) . build () . expect ("http client must build with success") }
};
}
