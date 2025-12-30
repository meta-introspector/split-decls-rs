// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl crypto :: TicketerFactory for Provider { fn ticketer (& self) -> Result < Arc < dyn TicketProducer > , Error > { Ok (Arc :: new (Ticketer)) } fn fips (& self) -> bool { false } }
};
}
