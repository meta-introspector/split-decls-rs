// Generated macro for to_bytes (function)
macro_rules! Depcrate_test_helpersto_bytes {
() => {
// Module: crate::test_helpers
// Provides: {"to_bytes"}
// Dependencies: {}
pub (crate) async fn to_bytes < T > (body : T) -> Result < Bytes , T :: Error > where T : http_body :: Body , { Ok (body . collect () . await ? . to_bytes ()) }
};
}
