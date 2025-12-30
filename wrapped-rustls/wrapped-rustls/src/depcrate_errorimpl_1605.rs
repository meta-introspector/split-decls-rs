// Generated macro for impl_1605 (impl)
macro_rules! Depcrate_errorimpl_1605 {
() => {
// Module: crate::error
// Provides: {"impl_1605"}
// Dependencies: {}
impl From < InvalidMessage > for AlertDescription { fn from (e : InvalidMessage) -> Self { match e { InvalidMessage :: PreSharedKeyIsNotFinalExtension => Self :: IllegalParameter , InvalidMessage :: DuplicateExtension (_) => Self :: IllegalParameter , InvalidMessage :: UnknownHelloRetryRequestExtension => Self :: UnsupportedExtension , _ => Self :: DecodeError , } } }
};
}
