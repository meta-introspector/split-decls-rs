// Generated macro for skip (function)
macro_rules! Depcrate_trust_anchorskip {
() => {
// Module: crate::trust_anchor
// Provides: {"skip"}
// Dependencies: {}
fn skip (input : & mut untrusted :: Reader < '_ > , tag : der :: Tag) -> Result < () , Error > { der :: expect_tag (input , tag) . map (| _ | ()) }
};
}
