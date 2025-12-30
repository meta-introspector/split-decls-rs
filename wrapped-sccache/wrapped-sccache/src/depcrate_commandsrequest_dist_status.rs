// Generated macro for request_dist_status (function)
macro_rules! Depcrate_commandsrequest_dist_status {
() => {
// Module: crate::commands
// Provides: {"request_dist_status"}
// Dependencies: {}
# [doc = " Send a `DistStatus` request to the server, and return `DistStatus` if successful."] pub fn request_dist_status (mut conn : ServerConnection) -> Result < DistInfo > { debug ! ("request_dist_status") ; let response = conn . request (Request :: DistStatus) . context ("Failed to send data to or receive data from server") ? ; if let Response :: DistStatus (info) = response { Ok (info) } else { bail ! ("Unexpected server response!") } }
};
}
