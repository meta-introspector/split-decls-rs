// Generated macro for impl_191 (impl)
macro_rules! Depcrate_error_try_from_parsedimpl_191 {
() => {
// Module: crate::error::try_from_parsed
// Provides: {"impl_191"}
// Dependencies: {}
impl fmt :: Display for TryFromParsed { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: InsufficientInformation => f . write_str ("the `Parsed` struct did not include enough information to construct the type" ,) , Self :: ComponentRange (err) => err . fmt (f) , } } }
};
}
