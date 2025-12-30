// Generated macro for Extension (struct)
macro_rules! Depcrate_extExtension {
() => {
// Module: crate::ext
// Provides: {"Extension"}
// Dependencies: {}
# [doc = " Extension as defined in [RFC 5280 Section 4.1.2.9]."] # [doc = ""] # [doc = " The ASN.1 definition for Extension objects is below. The extnValue type"] # [doc = " may be further parsed using a decoder corresponding to the extnID value."] # [doc = ""] # [doc = " ```text"] # [doc = " Extension  ::=  SEQUENCE  {"] # [doc = "     extnID      OBJECT IDENTIFIER,"] # [doc = "     critical    BOOLEAN DEFAULT FALSE,"] # [doc = "     extnValue   OCTET STRING"] # [doc = "                 -- contains the DER encoding of an ASN.1 value"] # [doc = "                 -- corresponding to the extension type identified"] # [doc = "                 -- by extnID"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5280 Section 4.1.2.9]: https://datatracker.ietf.org/doc/html/rfc5280#section-4.1.2.9"] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] # [derive (Clone , Debug , Eq , PartialEq , Sequence , ValueOrd)] # [allow (missing_docs)] pub struct Extension { pub extn_id : ObjectIdentifier , # [asn1 (default = "Default::default")] pub critical : bool , pub extn_value : OctetString , }
};
}
