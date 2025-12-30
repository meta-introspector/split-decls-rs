// Generated macro for DistClientState (enum)
macro_rules! Depcrate_serverDistClientState {
() => {
// Module: crate::server
// Provides: {"DistClientState"}
// Dependencies: {}
# [cfg (feature = "dist-client")] pub enum DistClientState { # [cfg (feature = "dist-client")] Some (Box < DistClientConfig > , Arc < dyn dist :: Client >) , # [cfg (feature = "dist-client")] FailWithMessage (Box < DistClientConfig > , String) , # [cfg (feature = "dist-client")] RetryCreateAt (Box < DistClientConfig > , Instant) , Disabled , }
};
}
