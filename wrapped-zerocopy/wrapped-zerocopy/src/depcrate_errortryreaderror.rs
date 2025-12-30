// Generated macro for TryReadError (type)
macro_rules! Depcrate_errorTryReadError {
() => {
// Module: crate::error
// Provides: {"TryReadError"}
// Dependencies: {}
# [doc = " The error type of fallible read-conversions."] # [doc = ""] # [doc = " Fallible read-conversions, like [`TryFromBytes::try_read_from_bytes`] may emit"] # [doc = " [size](SizeError) and [validity](ValidityError) errors, but not alignment errors."] # [allow (type_alias_bounds)] pub type TryReadError < Src , Dst : ? Sized + TryFromBytes > = ConvertError < Infallible , SizeError < Src , Dst > , ValidityError < Src , Dst > > ;
};
}
