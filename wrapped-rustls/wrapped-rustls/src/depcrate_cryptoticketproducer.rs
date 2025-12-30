// Generated macro for TicketProducer (trait)
macro_rules! Depcrate_cryptoTicketProducer {
() => {
// Module: crate::crypto
// Provides: {"TicketProducer"}
// Dependencies: {}
# [doc = " A trait for the ability to encrypt and decrypt tickets."] pub trait TicketProducer : Debug + Send + Sync { # [doc = " Encrypt and authenticate `plain`, returning the resulting"] # [doc = " ticket.  Return None if `plain` cannot be encrypted for"] # [doc = " some reason: an empty ticket will be sent and the connection"] # [doc = " will continue."] fn encrypt (& self , plain : & [u8]) -> Option < Vec < u8 > > ; # [doc = " Decrypt `cipher`, validating its authenticity protection"] # [doc = " and recovering the plaintext.  `cipher` is fully attacker"] # [doc = " controlled, so this decryption must be side-channel free,"] # [doc = " panic-proof, and otherwise bullet-proof.  If the decryption"] # [doc = " fails, return None."] fn decrypt (& self , cipher : & [u8]) -> Option < Vec < u8 > > ; # [doc = " Returns the lifetime of tickets produced now."] # [doc = " The lifetime is provided as a hint to clients that the"] # [doc = " ticket will not be useful after the given time."] # [doc = ""] # [doc = " This lifetime must be implemented by key rolling and"] # [doc = " erasure, *not* by storing a lifetime in the ticket."] # [doc = ""] # [doc = " The objective is to limit damage to forward secrecy caused"] # [doc = " by tickets, not just limiting their lifetime."] fn lifetime (& self) -> Duration ; }
};
}
