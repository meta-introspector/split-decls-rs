// Generated macro for Attribute (struct)
macro_rules! Depcrate_attrAttribute {
() => {
// Module: crate::attr
// Provides: {"Attribute"}
// Dependencies: {}
# [doc = " X.501 `Attribute` as defined in [RFC 5280 Appendix A.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " Attribute               ::= SEQUENCE {"] # [doc = "     type             AttributeType,"] # [doc = "     values    SET OF AttributeValue -- at least one value is required"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Note that [RFC 2986 Section 4] defines a constrained version of this type:"] # [doc = ""] # [doc = " ```text"] # [doc = " Attribute { ATTRIBUTE:IOSet } ::= SEQUENCE {"] # [doc = "     type   ATTRIBUTE.&id({IOSet}),"] # [doc = "     values SET SIZE(1..MAX) OF ATTRIBUTE.&Type({IOSet}{@type})"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " The unconstrained version should be preferred."] # [doc = ""] # [doc = " [RFC 2986 Section 4]: https://datatracker.ietf.org/doc/html/rfc2986#section-4"] # [doc = " [RFC 5280 Appendix A.1]: https://datatracker.ietf.org/doc/html/rfc5280#appendix-A.1"] # [derive (Clone , Debug , PartialEq , Eq , Sequence , ValueOrd)] # [allow (missing_docs)] pub struct Attribute { pub oid : AttributeType , pub values : SetOfVec < AttributeValue > , }
};
}
