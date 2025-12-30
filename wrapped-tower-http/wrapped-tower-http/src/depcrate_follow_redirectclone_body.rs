// Generated macro for clone_body (function)
macro_rules! Depcrate_follow_redirectclone_body {
() => {
// Module: crate::follow_redirect
// Provides: {"clone_body"}
// Dependencies: {}
fn clone_body < P , B , E > (policy : & P , body : & B) -> Option < B > where P : Policy < B , E > , B : Body + Default , { if body . size_hint () . exact () == Some (0) { Some (B :: default ()) } else { policy . clone_body (body) } }
};
}
