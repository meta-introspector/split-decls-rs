// Generated macro for impl_44 (impl)
macro_rules! Depcrate_hostimpl_44 {
() => {
// Module: crate::host
// Provides: {"impl_44"}
// Dependencies: {}
impl Host < String > { # [doc = " Parse a host: either an IPv6 address in [] square brackets, or a domain."] # [doc = ""] # [doc = " <https://url.spec.whatwg.org/#host-parsing>"] pub fn parse (input : & str) -> Result < Self , ParseError > { Host :: < Cow < str > > :: parse_cow (input . into ()) . map (| i | i . into_owned ()) } # [doc = " <https://url.spec.whatwg.org/#concept-opaque-host-parser>"] pub fn parse_opaque (input : & str) -> Result < Self , ParseError > { Host :: < Cow < str > > :: parse_opaque_cow (input . into ()) . map (| i | i . into_owned ()) } }
};
}
