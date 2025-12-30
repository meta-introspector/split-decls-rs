// Generated macro for impl_443 (impl)
macro_rules! Depcrate_deimpl_443 {
() => {
// Module: crate::de
// Provides: {"impl_443"}
// Dependencies: {}
# [cfg (feature = "parse")] impl < S : AsRef < str > > Deserializer < S > { # [doc = " Parse a TOML document"] pub fn parse (raw : S) -> Result < Self , Error > { crate :: Document :: parse (raw) . map (Self :: from) . map_err (Into :: into) } }
};
}
