// Generated macro for Http3Settings (struct)
macro_rules! Depcrate_http3_settingsHttp3Settings {
() => {
// Module: crate::http3::settings
// Provides: {"Http3Settings"}
// Dependencies: {}
# [doc = " Unified configuration parameters for"] # [doc = " [H3Driver](crate::http3::driver::H3Driver)s."] # [derive (Default , Clone , Debug)] pub struct Http3Settings { # [doc = " Maximum number of requests a"] # [doc = " [ServerH3Driver](crate::http3::driver::ServerH3Driver) allows per"] # [doc = " connection."] pub max_requests_per_connection : Option < u64 > , # [doc = " Maximum size of a single HEADERS frame, in bytes."] pub max_header_list_size : Option < u64 > , # [doc = " Maximum value the QPACK encoder is permitted to set for the dynamic"] # [doc = " table capcity. See <https://www.rfc-editor.org/rfc/rfc9204.html#name-maximum-dynamic-table-capac>"] pub qpack_max_table_capacity : Option < u64 > , # [doc = " Upper bound on the number of streams that can be blocked on the QPACK"] # [doc = " decoder. See <https://www.rfc-editor.org/rfc/rfc9204.html#name-blocked-streams>"] pub qpack_blocked_streams : Option < u64 > , # [doc = " Timeout between starting the QUIC handshake and receiving the first"] # [doc = " request on a connection. Only applicable to"] # [doc = " [ServerH3Driver](crate::http3::driver::ServerH3Driver)."] pub post_accept_timeout : Option < Duration > , # [doc = " Set the `SETTINGS_ENABLE_CONNECT_PROTOCOL` HTTP/3 setting."] # [doc = " See <https://www.rfc-editor.org/rfc/rfc9220#section-3-2>"] pub enable_extended_connect : bool , }
};
}
