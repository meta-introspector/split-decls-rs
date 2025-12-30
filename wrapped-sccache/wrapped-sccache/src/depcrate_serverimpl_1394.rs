// Generated macro for impl_1394 (impl)
macro_rules! Depcrate_serverimpl_1394 {
() => {
// Module: crate::server
// Provides: {"impl_1394"}
// Dependencies: {}
# [cfg (not (feature = "dist-client"))] impl DistClientContainer { # [cfg (not (feature = "dist-client"))] fn new (config : & Config , _ : & tokio :: runtime :: Handle) -> Self { if config . dist . scheduler_url . is_some () { warn ! ("Scheduler address configured but dist feature disabled, disabling distributed sccache") } Self { } } pub fn new_disabled () -> Self { Self { } } # [cfg (feature = "dist-client")] pub fn new_with_state (_ : DistClientState) -> Self { Self { } } pub async fn reset_state (& self) { } pub async fn get_status (& self) -> DistInfo { DistInfo :: Disabled ("dist-client feature not selected" . to_string ()) } async fn get_client (& self) -> Result < Option < Arc < dyn dist :: Client > > > { Ok (None) } }
};
}
