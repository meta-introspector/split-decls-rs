// Generated macro for event_channel (function)
macro_rules! Depcrateevent_channel {
() => {
// Module: crate
// Provides: {"event_channel"}
// Dependencies: {}
fn event_channel () -> (mpsc :: Sender < Option < LokiEvent > > , mpsc :: Receiver < Option < LokiEvent > > ,) { mpsc :: channel (512) }
};
}
