// Generated macro for ServiceLocator (struct)
macro_rules! Depcrate_extServiceLocator {
() => {
// Module: crate::ext
// Provides: {"ServiceLocator"}
// Dependencies: {}
# [doc = " ServiceLocator structure as defined in [RFC 6960 Section 4.4.6]."] # [doc = ""] # [doc = " ```text"] # [doc = " ServiceLocator ::= SEQUENCE {"] # [doc = "    issuer                  Name,"] # [doc = "    locator                 AuthorityInfoAccessSyntax }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6960 Section 4.4.6]: https://datatracker.ietf.org/doc/html/rfc6960#section-4.4.6"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct ServiceLocator { pub issuer : Name , pub locator : Option < AuthorityInfoAccessSyntax > , }
};
}
