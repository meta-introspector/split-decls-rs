// Generated macro for PkiPath (type)
macro_rules! Depcrate_certificatePkiPath {
() => {
// Module: crate::certificate
// Provides: {"PkiPath"}
// Dependencies: {}
# [doc = " `PkiPath` as defined by X.509 and referenced by [RFC 6066]."] # [doc = ""] # [doc = " This contains a series of certificates in validation order from the"] # [doc = " top-most certificate to the bottom-most certificate. This means that"] # [doc = " the first certificate signs the second certificate and so on."] # [doc = ""] # [doc = " ```text"] # [doc = " PkiPath ::= SEQUENCE OF Certificate"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6066]: https://datatracker.ietf.org/doc/html/rfc6066#section-10.1"] pub type PkiPath = Vec < Certificate > ;
};
}
