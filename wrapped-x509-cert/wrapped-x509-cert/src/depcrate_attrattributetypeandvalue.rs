// Generated macro for AttributeTypeAndValue (struct)
macro_rules! Depcrate_attrAttributeTypeAndValue {
() => {
// Module: crate::attr
// Provides: {"AttributeTypeAndValue"}
// Dependencies: {}
# [doc = " X.501 `AttributeTypeAndValue` as defined in [RFC 5280 Appendix A.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " AttributeTypeAndValue ::= SEQUENCE {"] # [doc = "   type     AttributeType,"] # [doc = "   value    AttributeValue"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5280 Appendix A.1]: https://datatracker.ietf.org/doc/html/rfc5280#appendix-A.1"] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] # [derive (Clone , Debug , Eq , PartialEq , PartialOrd , Ord , Sequence , ValueOrd , Hash)] # [allow (missing_docs)] pub struct AttributeTypeAndValue { pub oid : AttributeType , pub value : AttributeValue , }
};
}
