// Generated macro for impl_138 (impl)
macro_rules! Depcrate_error_formatimpl_138 {
() => {
// Module: crate::error::format
// Provides: {"impl_138"}
// Dependencies: {}
impl core :: error :: Error for Format { # [inline] fn source (& self) -> Option < & (dyn core :: error :: Error + 'static) > { match self { Self :: InsufficientTypeInformation | Self :: InvalidComponent (_) => None , Self :: ComponentRange (err) => Some (& * * err) , Self :: StdIo (err) => Some (err) , } } }
};
}
