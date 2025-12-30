// Generated macro for impl_194 (impl)
macro_rules! Depcrate_error_try_from_parsedimpl_194 {
() => {
// Module: crate::error::try_from_parsed
// Provides: {"impl_194"}
// Dependencies: {}
impl core :: error :: Error for TryFromParsed { # [inline] fn source (& self) -> Option < & (dyn core :: error :: Error + 'static) > { match self { Self :: InsufficientInformation => None , Self :: ComponentRange (err) => Some (err) , } } }
};
}
