// Generated macro for H3Controller (struct)
macro_rules! Depcrate_http3_driverH3Controller {
() => {
// Module: crate::http3::driver
// Provides: {"H3Controller"}
// Dependencies: {}
# [doc = " Interface to communicate with a paired [H3Driver]."] # [doc = ""] # [doc = " An [H3Controller] receives [`H3Event`]s from its driver, which must be"] # [doc = " consumed by the application built on top of the driver to react to incoming"] # [doc = " events. The controller also allows the application to send ad-hoc"] # [doc = " [`H3Command`]s to the driver, which will be processed when the driver waits"] # [doc = " for incoming data."] pub struct H3Controller < H : DriverHooks > { # [doc = " Sends [`H3Command`]s to the [H3Driver], like [`QuicCommand`]s or"] # [doc = " outbound HTTP requests."] cmd_sender : UnboundedSender < H :: Command > , # [doc = " Receives [`H3Event`]s from the [H3Driver]. Can be extracted and"] # [doc = " used independently of the [H3Controller]."] h3_event_recv : Option < UnboundedReceiver < H :: Event > > , }
};
}
