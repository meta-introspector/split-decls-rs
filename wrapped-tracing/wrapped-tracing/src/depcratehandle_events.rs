// Generated macro for handle_events (function)
macro_rules! Depcratehandle_events {
() => {
// Module: crate
// Provides: {"handle_events"}
// Dependencies: {}
# [doc = " Handle events and insert them into the events vector keeping only the last 10 events"] # [instrument (skip (events))] fn handle_events (events : & mut Vec < Event >) -> Result < () > { if event :: poll (Duration :: from_millis (100)) ? { let event = event :: read () ? ; debug ! (? event) ; events . insert (0 , event) ; } events . truncate (10) ; Ok (()) }
};
}
