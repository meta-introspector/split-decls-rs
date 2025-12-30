// Generated macro for impl_27 (impl)
macro_rules! Depcrate_errorimpl_27 {
() => {
// Module: crate::error
// Provides: {"impl_27"}
// Dependencies: {}
impl From < der :: Error > for Error { fn from (err : der :: Error) -> Error { if let der :: ErrorKind :: OidUnknown { oid } = err . kind () { Error :: OidUnknown { oid } } else { Error :: Asn1 (err) } } }
};
}
