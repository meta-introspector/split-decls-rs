// Generated macro for Networks (struct)
macro_rules! Depcrate_common_networkNetworks {
() => {
// Module: crate::common::network
// Provides: {"Networks"}
// Dependencies: {}
# [doc = " Interacting with network interfaces."] # [doc = ""] # [doc = " ```no_run"] # [doc = " use sysinfo::Networks;"] # [doc = ""] # [doc = " let networks = Networks::new_with_refreshed_list();"] # [doc = " for (interface_name, network) in &networks {"] # [doc = "     println!(\"[{interface_name}]: {network:?}\");"] # [doc = " }"] # [doc = " ```"] pub struct Networks { pub (crate) inner : NetworksInner , }
};
}
