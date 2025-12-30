// Generated macro for create_empty_array (function)
macro_rules! Depcrate_network_configurationcreate_empty_array {
() => {
// Module: crate::network_configuration
// Provides: {"create_empty_array"}
// Dependencies: {}
fn create_empty_array < T > () -> CFArray < T > { use std :: ptr :: null ; unsafe { CFArray :: wrap_under_create_rule (core_foundation :: array :: CFArrayCreate (null () as * const _ , null () as * const _ , 0 , null () as * const _ ,)) } }
};
}
