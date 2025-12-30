// Generated macro for ValidUses (enum)
macro_rules! Depcrate_cert_contextValidUses {
() => {
// Module: crate::cert_context
// Provides: {"ValidUses"}
// Dependencies: {}
# [doc = " Valid uses of a Certificate - All, or specific OIDs"] pub enum ValidUses { # [doc = " Certificate is valid for all uses"] All , # [doc = " Certificate is valid for uses specified. No entries means that the certificate"] # [doc = " has no valid uses."] Oids (Vec < String >) , }
};
}
