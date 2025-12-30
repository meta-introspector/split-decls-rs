// Generated macro for impl_289 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_289 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_289"}
// Dependencies: {}
impl ServerExtensions < '_ > { fn into_owned (self) -> ServerExtensions < 'static > { let Self { ec_point_formats , server_name_ack , session_ticket_ack , renegotiation_info , selected_protocol , key_share , preshared_key , client_certificate_type , server_certificate_type , extended_master_secret_ack , certificate_status_request_ack , selected_version , transport_parameters , early_data_ack , encrypted_client_hello_ack , unknown_extensions , } = self ; ServerExtensions { ec_point_formats , server_name_ack , session_ticket_ack , renegotiation_info , selected_protocol , key_share , preshared_key , client_certificate_type , server_certificate_type , extended_master_secret_ack , certificate_status_request_ack , selected_version , transport_parameters : transport_parameters . map (| x | x . into_owned ()) , early_data_ack , encrypted_client_hello_ack , unknown_extensions , } } }
};
}
