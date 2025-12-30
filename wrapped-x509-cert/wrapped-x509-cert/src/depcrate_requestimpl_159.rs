// Generated macro for impl_159 (impl)
macro_rules! Depcrate_requestimpl_159 {
() => {
// Module: crate::request
// Provides: {"impl_159"}
// Dependencies: {}
impl TryFrom < ExtensionReq > for Attribute { type Error = der :: Error ; fn try_from (extension_req : ExtensionReq) -> der :: Result < Attribute > { let mut values : SetOfVec < AttributeValue > = Default :: default () ; values . insert (Any :: encode_from (& extension_req . 0) ?) ? ; Ok (Attribute { oid : ExtensionReq :: OID , values , }) } }
};
}
