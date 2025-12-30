// Generated macro for impl_28 (impl)
macro_rules! Depcrate_versioned_sanitizedimpl_28 {
() => {
// Module: crate::versioned::sanitized
// Provides: {"impl_28"}
// Dependencies: {}
impl SanitizedVersionedTransaction { pub fn try_new (tx : VersionedTransaction) -> Result < Self , SanitizeError > { tx . sanitize_signatures () ? ; Ok (Self { signatures : tx . signatures , message : SanitizedVersionedMessage :: try_from (tx . message) ? , }) } pub fn get_message (& self) -> & SanitizedVersionedMessage { & self . message } # [doc = " Consumes the SanitizedVersionedTransaction, returning the fields individually."] pub fn destruct (self) -> (Vec < Signature > , SanitizedVersionedMessage) { (self . signatures , self . message) } }
};
}
