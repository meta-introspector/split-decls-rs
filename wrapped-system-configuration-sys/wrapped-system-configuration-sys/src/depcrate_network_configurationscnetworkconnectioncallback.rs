// Generated macro for SCNetworkConnectionCallBack (type)
macro_rules! Depcrate_network_configurationSCNetworkConnectionCallBack {
() => {
// Module: crate::network_configuration
// Provides: {"SCNetworkConnectionCallBack"}
// Dependencies: {}
pub type SCNetworkConnectionCallBack = Option < unsafe extern "C" fn (connection : SCNetworkConnectionRef , status : SCNetworkConnectionStatus , info : * mut :: core :: ffi :: c_void ,) , > ;
};
}
