// Generated macro for update_network_data (function)
macro_rules! Depcrate_unix_apple_networkupdate_network_data {
() => {
// Module: crate::unix::apple::network
// Provides: {"update_network_data"}
// Dependencies: {}
fn update_network_data (inner : & mut NetworkDataInner , data : & if_data64) { update_field (& mut inner . old_out , & mut inner . current_out , data . ifi_obytes) ; update_field (& mut inner . old_in , & mut inner . current_in , data . ifi_ibytes) ; update_field (& mut inner . old_packets_out , & mut inner . packets_out , data . ifi_opackets ,) ; update_field (& mut inner . old_packets_in , & mut inner . packets_in , data . ifi_ipackets ,) ; update_field (& mut inner . old_errors_in , & mut inner . errors_in , data . ifi_ierrors ,) ; update_field (& mut inner . old_errors_out , & mut inner . errors_out , data . ifi_oerrors ,) ; }
};
}
