// Generated macro for TicketRotator (struct)
macro_rules! Depcrate_ticketerTicketRotator {
() => {
// Module: crate::ticketer
// Provides: {"TicketRotator"}
// Dependencies: {}
# [doc = " A ticketer that has a 'current' sub-ticketer and a single"] # [doc = " 'previous' ticketer.  It creates a new ticketer every so"] # [doc = " often, demoting the current ticketer."] # [cfg (feature = "std")] pub struct TicketRotator { pub (crate) generator : fn () -> Result < Box < dyn TicketProducer > , Error > , lifetime : Duration , state : RwLock < TicketRotatorState > , }
};
}
