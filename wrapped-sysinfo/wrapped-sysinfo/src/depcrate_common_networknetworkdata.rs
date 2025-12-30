// Generated macro for NetworkData (struct)
macro_rules! Depcrate_common_networkNetworkData {
() => {
// Module: crate::common::network
// Provides: {"NetworkData"}
// Dependencies: {}
# [doc = " Getting volume of received and transmitted data."] # [doc = ""] # [doc = " ```no_run"] # [doc = " use sysinfo::Networks;"] # [doc = ""] # [doc = " let networks = Networks::new_with_refreshed_list();"] # [doc = " for (interface_name, network) in &networks {"] # [doc = "     println!(\"[{interface_name}] {network:?}\");"] # [doc = " }"] # [doc = " ```"] pub struct NetworkData { pub (crate) inner : NetworkDataInner , }
};
}
