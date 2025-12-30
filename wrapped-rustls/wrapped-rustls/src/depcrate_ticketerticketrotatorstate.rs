// Generated macro for TicketRotatorState (struct)
macro_rules! Depcrate_ticketerTicketRotatorState {
() => {
// Module: crate::ticketer
// Provides: {"TicketRotatorState"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct TicketRotatorState { current : Box < dyn TicketProducer > , previous : Option < Box < dyn TicketProducer > > , next_switch_time : u64 , }
};
}
