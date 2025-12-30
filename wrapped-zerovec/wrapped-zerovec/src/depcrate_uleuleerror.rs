// Generated macro for UleError (enum)
macro_rules! Depcrate_uleUleError {
() => {
// Module: crate::ule
// Provides: {"UleError"}
// Dependencies: {}
# [doc = " An error type to be used for decoding slices of ULE types"] # [derive (Copy , Clone , Debug , PartialEq , Eq)] # [non_exhaustive] pub enum UleError { # [doc = " Attempted to parse a buffer into a slice of the given ULE type but its"] # [doc = " length was not compatible."] # [doc = ""] # [doc = " Typically created by a [`ULE`] impl via [`UleError::length()`]."] # [doc = ""] # [doc = " [`ULE`]: crate::ule::ULE"] InvalidLength { ty : & 'static str , len : usize } , # [doc = " The byte sequence provided for `ty` failed to parse correctly in the"] # [doc = " given ULE type."] # [doc = ""] # [doc = " Typically created by a [`ULE`] impl via [`UleError::parse()`]."] # [doc = ""] # [doc = " [`ULE`]: crate::ule::ULE"] ParseError { ty : & 'static str } , }
};
}
