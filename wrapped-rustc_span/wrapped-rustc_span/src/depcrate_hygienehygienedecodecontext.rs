// Generated macro for HygieneDecodeContext (struct)
macro_rules! Depcrate_hygieneHygieneDecodeContext {
() => {
// Module: crate::hygiene
// Provides: {"HygieneDecodeContext"}
// Dependencies: {}
# [doc = " Additional information used to assist in decoding hygiene data"] # [derive (Default)] pub struct HygieneDecodeContext { remapped_ctxts : Lock < IndexVec < u32 , Option < SyntaxContext > > > , }
};
}
