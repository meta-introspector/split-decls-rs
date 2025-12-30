// Generated macro for ServerMessage (enum)
macro_rules! Depcrate_serverServerMessage {
() => {
// Module: crate::server
// Provides: {"ServerMessage"}
// Dependencies: {}
# [doc = " Messages sent from all services to the main event loop indicating activity."] # [doc = ""] # [doc = " Whenever a request is receive a `Request` message is sent which will reset"] # [doc = " the idle shutdown timer, and otherwise a `Shutdown` message indicates that"] # [doc = " a server shutdown was requested via an RPC."] pub enum ServerMessage { # [doc = " A message sent whenever a request is received."] Request , # [doc = " Message sent whenever a shutdown request is received."] Shutdown , }
};
}
