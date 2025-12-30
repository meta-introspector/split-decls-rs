// Generated macro for Client (struct)
macro_rules! DepcrateClient {
() => {
// Module: crate
// Provides: {"Client"}
// Dependencies: {}
struct Client { uaid : Uuid , use_webpush : bool , channel_ids : Vec < Uuid > , tx : mpsc :: UnboundedSender < Notification > , }
};
}
