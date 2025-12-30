// Generated macro for request_shutdown (function)
macro_rules! Depcrate_commandsrequest_shutdown {
() => {
// Module: crate::commands
// Provides: {"request_shutdown"}
// Dependencies: {}
# [doc = " Send a `Shutdown` request to the server, and return the `ServerInfo` contained within the response if successful."] pub fn request_shutdown (mut conn : ServerConnection) -> Result < ServerInfo > { debug ! ("request_shutdown") ; let response = conn . request (Request :: Shutdown) . context ("Failed to send data to or receive data from server") ? ; if let Response :: ShuttingDown (stats) = response { Ok (* stats) } else { bail ! ("Unexpected server response!") } }
};
}
