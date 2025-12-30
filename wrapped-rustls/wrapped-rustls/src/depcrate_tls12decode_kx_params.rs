// Generated macro for decode_kx_params (function)
macro_rules! Depcrate_tls12decode_kx_params {
() => {
// Module: crate::tls12
// Provides: {"decode_kx_params"}
// Dependencies: {}
pub (crate) fn decode_kx_params < 'a , T : KxDecode < 'a > > (kx_algorithm : KeyExchangeAlgorithm , common : & mut CommonState , kx_params : & 'a [u8] ,) -> Result < T , Error > { let mut rd = Reader :: init (kx_params) ; let kx_params = T :: decode (& mut rd , kx_algorithm) ? ; match rd . any_left () { false => Ok (kx_params) , true => Err (common . send_fatal_alert (AlertDescription :: DecodeError , InvalidMessage :: InvalidDhParams ,)) , } }
};
}
