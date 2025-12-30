// Generated macro for CastError (type)
macro_rules! Depcrate_errorCastError {
() => {
// Module: crate::error
// Provides: {"CastError"}
// Dependencies: {}
# [doc = " The error type of reference conversions."] # [doc = ""] # [doc = " Reference conversions, like [`FromBytes::ref_from_bytes`] may emit"] # [doc = " [alignment](AlignmentError) and [size](SizeError) errors."] # [allow (type_alias_bounds)] pub type CastError < Src , Dst : ? Sized > = ConvertError < AlignmentError < Src , Dst > , SizeError < Src , Dst > , Infallible > ;
};
}
