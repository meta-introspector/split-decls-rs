// Generated macro for CrlDistributionPoint (struct)
macro_rules! Depcrate_certCrlDistributionPoint {
() => {
// Module: crate::cert
// Provides: {"CrlDistributionPoint"}
// Dependencies: {}
# [doc = " A certificate revocation list (CRL) distribution point, describing a source of"] # [doc = " CRL information for a given certificate as described in RFC 5280 section 4.2.3.13[^1]."] # [doc = ""] # [doc = " [^1]: <https://datatracker.ietf.org/doc/html/rfc5280#section-4.2.1.13>"] pub (crate) struct CrlDistributionPoint < 'a > { # [doc = " distributionPoint describes the location of CRL information."] distribution_point : Option < untrusted :: Input < 'a > > , # [doc = " reasons holds a bit flag set of certificate revocation reasons associated with the"] # [doc = " CRL distribution point."] pub (crate) reasons : Option < der :: BitStringFlags < 'a > > , # [doc = " when the CRL issuer is not the certificate issuer, crl_issuer identifies the issuer of the"] # [doc = " CRL."] pub (crate) crl_issuer : Option < untrusted :: Input < 'a > > , }
};
}
