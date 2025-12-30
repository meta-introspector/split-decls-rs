// Generated macro for JobAuthorizer (trait)
macro_rules! Depcrate_distJobAuthorizer {
() => {
// Module: crate::dist
// Provides: {"JobAuthorizer"}
// Dependencies: {}
# [cfg (feature = "dist-server")] pub trait JobAuthorizer : Send { fn generate_token (& self , job_id : JobId) -> Result < String > ; fn verify_token (& self , job_id : JobId , token : & str) -> Result < () > ; }
};
}
