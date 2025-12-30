// Generated macro for impl_1429 (impl)
macro_rules! Depcrate_crypto_cipherimpl_1429 {
() => {
// Module: crate::crypto::cipher
// Provides: {"impl_1429"}
// Dependencies: {}
impl PlainMessage { # [doc = " Construct by decoding from a [`Reader`]."] # [doc = ""] # [doc = " `MessageError` allows callers to distinguish between valid prefixes (might"] # [doc = " become valid if we read more data) and invalid data."] pub fn read (r : & mut Reader < '_ >) -> Result < Self , MessageError > { let (typ , version , len) = read_opaque_message_header (r) ? ; let content = r . take (len as usize) . ok_or (MessageError :: TooShortForLength) ? ; Ok (Self { typ , version , payload : Payload :: Owned (content . to_vec ()) , }) } # [doc = " Convert into an unencrypted [`OutboundOpaqueMessage`] (without decrypting)."] pub fn into_unencrypted_opaque (self) -> OutboundOpaqueMessage { OutboundOpaqueMessage { version : self . version , typ : self . typ , payload : PrefixedPayload :: from (self . payload . bytes ()) , } } # [doc = " Borrow as an [`InboundPlainMessage`]."] pub fn borrow_inbound (& self) -> InboundPlainMessage < '_ > { InboundPlainMessage { version : self . version , typ : self . typ , payload : self . payload . bytes () , } } # [doc = " Borrow as an [`OutboundPlainMessage`]."] pub fn borrow_outbound (& self) -> OutboundPlainMessage < '_ > { OutboundPlainMessage { version : self . version , typ : self . typ , payload : self . payload . bytes () . into () , } } }
};
}
