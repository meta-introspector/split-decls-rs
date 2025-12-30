// Generated macro for Attributes (type)
macro_rules! Depcrate_attrAttributes {
() => {
// Module: crate::attr
// Provides: {"Attributes"}
// Dependencies: {}
# [doc = " X.501 `Attributes` as defined in [RFC 2986 Section 4]."] # [doc = ""] # [doc = " ```text"] # [doc = " Attributes { ATTRIBUTE:IOSet } ::= SET OF Attribute{{ IOSet }}"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 2986 Section 4]: https://datatracker.ietf.org/doc/html/rfc2986#section-4"] pub type Attributes = SetOfVec < Attribute > ;
};
}
