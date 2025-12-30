// Generated macro for usable_for_rustls (function)
macro_rules! Depcrate_windowsusable_for_rustls {
() => {
// Module: crate::windows
// Provides: {"usable_for_rustls"}
// Dependencies: {}
fn usable_for_rustls (uses : ValidUses) -> bool { match uses { ValidUses :: All => true , ValidUses :: Oids (strs) => strs . iter () . any (| x | x == PKIX_SERVER_AUTH) , } }
};
}
