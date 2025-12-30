// Generated macro for plaintext (module)
macro_rules! Depcrateplaintext {
() => {
// Module: crate
// Provides: {"plaintext"}
// Dependencies: {}
mod plaintext { use rustls :: ConnectionTrafficSecrets ; use rustls :: crypto :: cipher :: { AeadKey , InboundOpaqueMessage , InboundPlainMessage , Iv , MessageDecrypter , MessageEncrypter , OutboundPlainMessage , PrefixedPayload , Tls13AeadAlgorithm , UnsupportedOperationError , } ; use super :: * ; pub (super) struct Aead ; impl Tls13AeadAlgorithm for Aead { fn encrypter (& self , _key : AeadKey , _iv : Iv) -> Box < dyn MessageEncrypter > { Box :: new (Encrypter) } fn decrypter (& self , _key : AeadKey , _iv : Iv) -> Box < dyn MessageDecrypter > { Box :: new (Decrypter) } fn key_len (& self) -> usize { 32 } fn extract_keys (& self , _key : AeadKey , _iv : Iv ,) -> Result < ConnectionTrafficSecrets , UnsupportedOperationError > { Err (UnsupportedOperationError) } } struct Encrypter ; impl MessageEncrypter for Encrypter { fn encrypt (& mut self , msg : OutboundPlainMessage < '_ > , _seq : u64 ,) -> Result < OutboundOpaqueMessage , Error > { let mut payload = PrefixedPayload :: with_capacity (msg . payload . len ()) ; payload . extend_from_chunks (& msg . payload) ; Ok (OutboundOpaqueMessage { typ : ContentType :: ApplicationData , version : ProtocolVersion :: TLSv1_2 , payload , }) } fn encrypted_payload_len (& self , payload_len : usize) -> usize { payload_len } } struct Decrypter ; impl MessageDecrypter for Decrypter { fn decrypt < 'a > (& mut self , msg : InboundOpaqueMessage < 'a > , _seq : u64 ,) -> Result < InboundPlainMessage < 'a > , Error > { Ok (msg . into_plain_message ()) } } }
};
}
