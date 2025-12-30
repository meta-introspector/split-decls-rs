// Generated macro for TicketerFactory (trait)
macro_rules! Depcrate_cryptoTicketerFactory {
() => {
// Module: crate::crypto
// Provides: {"TicketerFactory"}
// Dependencies: {}
# [doc = " A factory that builds [`TicketProducer`]s."] # [doc = ""] # [doc = " These can be used in [`ServerConfig::ticketer`] to enable stateless resumption."] # [doc = ""] # [doc = " [`ServerConfig::ticketer`]: crate::server::ServerConfig::ticketer"] pub trait TicketerFactory : Debug + Send + Sync { # [doc = " Build a new `TicketProducer`."] fn ticketer (& self) -> Result < Arc < dyn TicketProducer > , Error > ; # [doc = " Return `true` if this is backed by a FIPS-approved implementation."] fn fips (& self) -> bool ; }
};
}
