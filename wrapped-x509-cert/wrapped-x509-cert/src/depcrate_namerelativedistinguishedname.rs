// Generated macro for RelativeDistinguishedName (struct)
macro_rules! Depcrate_nameRelativeDistinguishedName {
() => {
// Module: crate::name
// Provides: {"RelativeDistinguishedName"}
// Dependencies: {}
# [doc = " RelativeDistinguishedName as defined in [RFC 5280 Section 4.1.2.4]."] # [doc = ""] # [doc = " ```text"] # [doc = " RelativeDistinguishedName ::= SET SIZE (1..MAX) OF AttributeTypeAndValue"] # [doc = " ```"] # [doc = ""] # [doc = " Note that we follow the more common definition above. This technically"] # [doc = " differs from the definition in X.501, which is:"] # [doc = ""] # [doc = " ```text"] # [doc = " RelativeDistinguishedName ::= SET SIZE (1..MAX) OF AttributeTypeAndDistinguishedValue"] # [doc = ""] # [doc = " AttributeTypeAndDistinguishedValue ::= SEQUENCE {"] # [doc = "     type ATTRIBUTE.&id ({SupportedAttributes}),"] # [doc = "     value ATTRIBUTE.&Type({SupportedAttributes}{@type}),"] # [doc = "     primaryDistinguished BOOLEAN DEFAULT TRUE,"] # [doc = "     valuesWithContext SET SIZE (1..MAX) OF SEQUENCE {"] # [doc = "         distingAttrValue [0] ATTRIBUTE.&Type ({SupportedAttributes}{@type}) OPTIONAL,"] # [doc = "         contextList SET SIZE (1..MAX) OF Context"] # [doc = "     } OPTIONAL"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5280 Section 4.1.2.4]: https://datatracker.ietf.org/doc/html/rfc5280#section-4.1.2.4"] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] # [derive (Clone , Debug , Default , PartialEq , Eq , Hash)] pub struct RelativeDistinguishedName (pub (crate) SetOfVec < AttributeTypeAndValue >) ;
};
}
