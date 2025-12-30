// Generated macro for SanitizeMessageError (enum)
macro_rules! DepcrateSanitizeMessageError {
() => {
// Module: crate
// Provides: {"SanitizeMessageError"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] # [derive (PartialEq , Debug , Eq , Clone)] pub enum SanitizeMessageError { IndexOutOfBounds , ValueOutOfBounds , InvalidValue , AddressLoaderError (AddressLoaderError) , }
};
}
