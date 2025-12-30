// Generated macro for DIST_CLIENT_RECREATE_TIMEOUT (const)
macro_rules! Depcrate_serverDIST_CLIENT_RECREATE_TIMEOUT {
() => {
// Module: crate::server
// Provides: {"DIST_CLIENT_RECREATE_TIMEOUT"}
// Dependencies: {}
# [doc = " If the dist client couldn't be created, retry creation at this number"] # [doc = " of seconds from now (or later)"] # [cfg (feature = "dist-client")] const DIST_CLIENT_RECREATE_TIMEOUT : Duration = Duration :: from_secs (30) ;
};
}
