// Generated macro for SCNetworkInterfaceType (enum)
macro_rules! Depcrate_network_configurationSCNetworkInterfaceType {
() => {
// Module: crate::network_configuration
// Provides: {"SCNetworkInterfaceType"}
// Dependencies: {}
# [doc = " Represents the possible network interface types."] # [doc = ""] # [doc = " See [_Network Interface Types_] documentation for details."] # [doc = ""] # [doc = " [_Network Interface Types_]: https://developer.apple.com/documentation/systemconfiguration/scnetworkconfiguration/network_interface_types?language=objc"] # [derive (Debug)] pub enum SCNetworkInterfaceType { # [doc = " A 6to4 interface."] SixToFour , # [doc = " Bluetooth interface."] Bluetooth , # [doc = " Bridge interface."] Bridge , # [doc = " Ethernet bond interface."] Bond , # [doc = " Ethernet interface."] Ethernet , # [doc = " FireWire interface."] FireWire , # [doc = " IEEE80211 interface."] IEEE80211 , # [doc = " IPSec interface."] IPSec , # [doc = " IrDA interface."] IrDA , # [doc = " L2TP interface."] L2TP , # [doc = " Modem interface."] Modem , # [doc = " PPP interface."] PPP , # [doc = " PPTP interface."] # [doc = ""] # [doc = " Deprecated, one should use the PPP variant."] PPTP , # [doc = " Serial interface."] Serial , # [doc = " VLAN interface."] VLAN , # [doc = " WWAN interface."] WWAN , # [doc = " IPv4 interface."] IPv4 , }
};
}
