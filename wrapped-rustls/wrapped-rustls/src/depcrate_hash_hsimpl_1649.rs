// Generated macro for impl_1649 (impl)
macro_rules! Depcrate_hash_hsimpl_1649 {
() => {
// Module: crate::hash_hs
// Provides: {"impl_1649"}
// Dependencies: {}
impl HandshakeHashBuffer { pub (crate) fn new () -> Self { Self { buffer : Vec :: new () , client_auth_enabled : false , } } # [doc = " We might be doing client auth, so need to keep a full"] # [doc = " log of the handshake."] pub (crate) fn set_client_auth_enabled (& mut self) { self . client_auth_enabled = true ; } # [doc = " Hash/buffer a handshake message."] pub (crate) fn add_message (& mut self , m : & Message < '_ >) { match & m . payload { MessagePayload :: Handshake { encoded , .. } => self . add_raw (encoded . bytes ()) , MessagePayload :: HandshakeFlight (payload) => self . add_raw (payload . bytes ()) , _ => { } } ; } # [doc = " Hash or buffer a byte slice."] fn add_raw (& mut self , buf : & [u8]) { self . buffer . extend_from_slice (buf) ; } # [doc = " Get the hash value if we were to hash `extra` too."] pub (crate) fn hash_given (& self , provider : & 'static dyn hash :: Hash , extra : & [u8] ,) -> hash :: Output { let mut ctx = provider . start () ; ctx . update (& self . buffer) ; ctx . update (extra) ; ctx . finish () } # [doc = " We now know what hash function the verify_data will use."] pub (crate) fn start_hash (self , provider : & 'static dyn hash :: Hash) -> HandshakeHash { let mut ctx = provider . start () ; ctx . update (& self . buffer) ; HandshakeHash { provider , ctx , client_auth : match self . client_auth_enabled { true => Some (self . buffer) , false => None , } , } } }
};
}
