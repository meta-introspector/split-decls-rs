// Generated macro for impl_112 (impl)
macro_rules! Depcrate_nameimpl_112 {
() => {
// Module: crate::name
// Provides: {"impl_112"}
// Dependencies: {}
impl TryFrom < Vec < AttributeTypeAndValue > > for RelativeDistinguishedName { type Error = der :: Error ; fn try_from (vec : Vec < AttributeTypeAndValue >) -> der :: Result < RelativeDistinguishedName > { Ok (RelativeDistinguishedName (SetOfVec :: try_from (vec) ?)) } }
};
}
