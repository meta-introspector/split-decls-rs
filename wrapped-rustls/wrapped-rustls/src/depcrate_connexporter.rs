// Generated macro for Exporter (trait)
macro_rules! Depcrate_connExporter {
() => {
// Module: crate::conn
// Provides: {"Exporter"}
// Dependencies: {}
# [doc = " This trait is for any object that can export keying material."] # [doc = ""] # [doc = " The terminology comes from [RFC5705](https://datatracker.ietf.org/doc/html/rfc5705)"] # [doc = " but doesn't really involve \"exporting\" key material (in the usual meaning of \"export\""] # [doc = " -- of moving an artifact from one domain to another) but is best thought of as key"] # [doc = " diversification using an existing secret.  That secret is implicit in this interface,"] # [doc = " so is assumed to be held by `self`. The secret should be zeroized in `drop()`."] # [doc = ""] # [doc = " There are several such internal implementations, depending on the context"] # [doc = " and protocol version."] pub (crate) trait Exporter : Send + Sync { # [doc = " Fills in `output` with derived keying material."] # [doc = ""] # [doc = " This is deterministic depending on a base secret (implicit in `self`),"] # [doc = " plus the `label` and `context` values."] # [doc = ""] # [doc = " Must fill in `output` entirely, or return an error."] fn derive (& self , label : & [u8] , context : Option < & [u8] > , output : & mut [u8]) -> Result < () , Error > ; }
};
}
