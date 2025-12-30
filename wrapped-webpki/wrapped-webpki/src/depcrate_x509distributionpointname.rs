// Generated macro for DistributionPointName (enum)
macro_rules! Depcrate_x509DistributionPointName {
() => {
// Module: crate::x509
// Provides: {"DistributionPointName"}
// Dependencies: {}
# [doc = " A certificate revocation list (CRL) distribution point name, describing a source of"] # [doc = " CRL information for a given certificate as described in RFC 5280 section 4.2.3.13[^1]."] # [doc = ""] # [doc = " [^1]: <https://datatracker.ietf.org/doc/html/rfc5280#section-4.2.1.13>"] pub (crate) enum DistributionPointName < 'a > { # [doc = " The distribution point name is a relative distinguished name, relative to the CRL issuer."] NameRelativeToCrlIssuer , # [doc = " The distribution point name is a sequence of [GeneralName] items."] FullName (DerIterator < 'a , GeneralName < 'a > >) , }
};
}
