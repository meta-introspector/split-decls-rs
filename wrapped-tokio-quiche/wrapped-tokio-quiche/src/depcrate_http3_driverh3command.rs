// Generated macro for H3Command (enum)
macro_rules! Depcrate_http3_driverH3Command {
() => {
// Module: crate::http3::driver
// Provides: {"H3Command"}
// Dependencies: {}
# [doc = " [`H3Command`]s are sent by the [H3Controller] to alter the [H3Driver]'s"] # [doc = " state."] # [doc = ""] # [doc = " Both [ServerH3Driver] and [ClientH3Driver] may extend this enum with"] # [doc = " endpoint-specific variants."] # [derive (Debug)] pub enum H3Command { # [doc = " A connection-level command that executes directly on the"] # [doc = " [`quiche::Connection`]."] QuicCmd (QuicCommand) , # [doc = " Send a GOAWAY frame to the peer to initiate a graceful connection"] # [doc = " shutdown."] GoAway , }
};
}
