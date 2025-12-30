// Generated macro for impl_744 (impl)
macro_rules! Depcrate_connimpl_744 {
() => {
// Module: crate::conn
// Provides: {"impl_744"}
// Dependencies: {}
impl KeyingMaterialExporter { # [doc = " Derives key material from the agreed connection secrets."] # [doc = ""] # [doc = " This function fills in `output` with `output.len()` bytes of key"] # [doc = " material derived from a master connection secret using `label`"] # [doc = " and `context` for diversification. Ownership of the buffer is taken"] # [doc = " by the function and returned via the Ok result to ensure no key"] # [doc = " material leaks if the function fails."] # [doc = ""] # [doc = " See [RFC5705][] for more details on what this does and is for.  In"] # [doc = " other libraries this is often named `SSL_export_keying_material()`"] # [doc = " or `SslExportKeyingMaterial()`."] # [doc = ""] # [doc = " This function is not meaningful if `output.len()` is zero and will"] # [doc = " return an error in that case."] # [doc = ""] # [doc = " [RFC5705]: https://datatracker.ietf.org/doc/html/rfc5705"] pub fn derive < T : AsMut < [u8] > > (& self , label : & [u8] , context : Option < & [u8] > , mut output : T ,) -> Result < T , Error > { if output . as_mut () . is_empty () { return Err (ApiMisuse :: ExporterOutputZeroLength . into ()) ; } self . inner . derive (label , context , output . as_mut ()) . map (| _ | output) } }
};
}
