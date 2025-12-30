// Generated macro for ConvertError (enum)
macro_rules! Depcrate_errorConvertError {
() => {
// Module: crate::error
// Provides: {"ConvertError"}
// Dependencies: {}
# [doc = " Zerocopy's generic error type."] # [doc = ""] # [doc = " Generally speaking, zerocopy's conversions may fail for one of up to three"] # [doc = " reasons:"] # [doc = " - [`AlignmentError`]: the conversion source was improperly aligned"] # [doc = " - [`SizeError`]: the conversion source was of incorrect size"] # [doc = " - [`ValidityError`]: the conversion source contained invalid data"] # [doc = ""] # [doc = " However, not all conversions produce all errors. For instance,"] # [doc = " [`FromBytes::ref_from_bytes`] may fail due to alignment or size issues, but"] # [doc = " not validity issues. This generic error type captures these"] # [doc = " (im)possibilities via parameterization: `A` is parameterized with"] # [doc = " [`AlignmentError`], `S` is parameterized with [`SizeError`], and `V` is"] # [doc = " parameterized with [`Infallible`]."] # [doc = ""] # [doc = " Zerocopy never uses this type directly in its API. Rather, we provide three"] # [doc = " pre-parameterized aliases:"] # [doc = " - [`CastError`]: the error type of reference conversions"] # [doc = " - [`TryCastError`]: the error type of fallible reference conversions"] # [doc = " - [`TryReadError`]: the error type of fallible read conversions"] # [derive (PartialEq , Eq , Clone)] pub enum ConvertError < A , S , V > { # [doc = " The conversion source was improperly aligned."] Alignment (A) , # [doc = " The conversion source was of incorrect size."] Size (S) , # [doc = " The conversion source contained invalid data."] Validity (V) , }
};
}
