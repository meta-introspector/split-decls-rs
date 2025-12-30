// Generated macro for EcPrivateKey (struct)
macro_rules! Depcrate_private_keyEcPrivateKey {
() => {
// Module: crate::private_key
// Provides: {"EcPrivateKey"}
// Dependencies: {}
# [doc = " SEC1 elliptic curve private key."] # [doc = ""] # [doc = " Described in [SEC1: Elliptic Curve Cryptography (Version 2.0)]"] # [doc = " Appendix C.4 (p.108) and also [RFC5915 Section 3]:"] # [doc = ""] # [doc = " ```text"] # [doc = " ECPrivateKey ::= SEQUENCE {"] # [doc = "   version        INTEGER { ecPrivkeyVer1(1) } (ecPrivkeyVer1),"] # [doc = "   privateKey     OCTET STRING,"] # [doc = "   parameters [0] ECParameters {{ NamedCurve }} OPTIONAL,"] # [doc = "   publicKey  [1] BIT STRING OPTIONAL"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " When encoded as PEM (text), keys in this format begin with the following:"] # [doc = ""] # [doc = " ```text"] # [doc = " -----BEGIN EC PRIVATE KEY-----"] # [doc = " ```"] # [doc = ""] # [doc = " [SEC1: Elliptic Curve Cryptography (Version 2.0)]: https://www.secg.org/sec1-v2.pdf"] # [doc = " [RFC5915 Section 3]: https://datatracker.ietf.org/doc/html/rfc5915#section-3"] # [derive (Clone)] pub struct EcPrivateKey < 'a > { # [doc = " Private key data."] pub private_key : & 'a [u8] , # [doc = " Elliptic curve parameters."] pub parameters : Option < EcParameters > , # [doc = " Public key data, optionally available if version is V2."] pub public_key : Option < & 'a [u8] > , }
};
}
