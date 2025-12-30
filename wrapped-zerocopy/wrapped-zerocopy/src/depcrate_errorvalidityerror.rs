// Generated macro for ValidityError (struct)
macro_rules! Depcrate_errorValidityError {
() => {
// Module: crate::error
// Provides: {"ValidityError"}
// Dependencies: {}
# [doc = " The error emitted if the conversion source contains invalid data."] pub struct ValidityError < Src , Dst : ? Sized + TryFromBytes > { # [doc = " The source value involved in the conversion."] pub (crate) src : Src , # [doc = " The inner destination type involved in the conversion."] _dst : SendSyncPhantomData < Dst > , }
};
}
