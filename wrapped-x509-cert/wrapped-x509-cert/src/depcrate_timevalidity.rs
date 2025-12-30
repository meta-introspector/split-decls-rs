// Generated macro for Validity (struct)
macro_rules! Depcrate_timeValidity {
() => {
// Module: crate::time
// Provides: {"Validity"}
// Dependencies: {}
# [doc = " X.501 `Validity` as defined in [RFC 5280 Section 4.1.2.5]"] # [doc = ""] # [doc = " ```text"] # [doc = " Validity ::= SEQUENCE {"] # [doc = "     notBefore      Time,"] # [doc = "     notAfter       Time"] # [doc = " }"] # [doc = " ```"] # [doc = " [RFC 5280 Section 4.1.2.5]: https://datatracker.ietf.org/doc/html/rfc5280#section-4.1.2.5"] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] # [derive (Copy , Clone , Debug , Eq , PartialEq , ValueOrd)] pub struct Validity < P : Profile = Rfc5280 > { # [doc = " notBefore value"] pub not_before : Time , # [doc = " notAfter value"] pub not_after : Time , _profile : PhantomData < P > , }
};
}
