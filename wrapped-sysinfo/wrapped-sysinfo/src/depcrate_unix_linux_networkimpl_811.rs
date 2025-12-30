// Generated macro for impl_811 (impl)
macro_rules! Depcrate_unix_linux_networkimpl_811 {
() => {
// Module: crate::unix::linux::network
// Provides: {"impl_811"}
// Dependencies: {}
impl NetworksInner { pub (crate) fn new () -> Self { Self { interfaces : HashMap :: new () , } } pub (crate) fn list (& self) -> & HashMap < String , NetworkData > { & self . interfaces } pub (crate) fn refresh (& mut self , remove_not_listed_interfaces : bool) { refresh_networks_list_from_sysfs (& mut self . interfaces , remove_not_listed_interfaces , Path :: new ("/sys/class/net/") ,) ; refresh_networks_addresses (& mut self . interfaces) ; } }
};
}
