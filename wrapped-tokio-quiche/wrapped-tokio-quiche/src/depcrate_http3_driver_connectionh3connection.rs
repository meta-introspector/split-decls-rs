// Generated macro for H3Connection (struct)
macro_rules! Depcrate_http3_driver_connectionH3Connection {
() => {
// Module: crate::http3::driver::connection
// Provides: {"H3Connection"}
// Dependencies: {}
# [doc = " A wrapper for an h3-driven [QuicConnection] together with the driver's"] # [doc = " [H3Controller]."] pub struct H3Connection < H : DriverHooks > { pub quic_connection : QuicConnection , pub h3_controller : H3Controller < H > , }
};
}
