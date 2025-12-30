// Generated macro for macro_5661 (macro)
macro_rules! Depcrate_shared_netioapimacro_5661 {
() => {
// Module: crate::shared::netioapi
// Provides: {"macro_5661"}
// Dependencies: {}
STRUCT ! { struct MIB_IPINTERFACE_ROW { Family : ADDRESS_FAMILY , InterfaceLuid : NET_LUID , InterfaceIndex : NET_IFINDEX , MaxReassemblySize : ULONG , InterfaceIdentifier : ULONG64 , MinRouterAdvertisementInterval : ULONG , MaxRouterAdvertisementInterval : ULONG , AdvertisingEnabled : BOOLEAN , ForwardingEnabled : BOOLEAN , WeakHostSend : BOOLEAN , WeakHostReceive : BOOLEAN , UseAutomaticMetric : BOOLEAN , UseNeighborUnreachabilityDetection : BOOLEAN , ManagedAddressConfigurationSupported : BOOLEAN , OtherStatefulConfigurationSupported : BOOLEAN , AdvertiseDefaultRoute : BOOLEAN , RouterDiscoveryBehavior : NL_ROUTER_DISCOVERY_BEHAVIOR , DadTransmits : ULONG , BaseReachableTime : ULONG , RetransmitTime : ULONG , PathMtuDiscoveryTimeout : ULONG , LinkLocalAddressBehavior : NL_LINK_LOCAL_ADDRESS_BEHAVIOR , LinkLocalAddressTimeout : ULONG , ZoneIndices : [ULONG ; ScopeLevelCount as usize] , SitePrefixLength : ULONG , Metric : ULONG , NlMtu : ULONG , Connected : BOOLEAN , SupportsWakeUpPatterns : BOOLEAN , SupportsNeighborDiscovery : BOOLEAN , SupportsRouterDiscovery : BOOLEAN , ReachableTime : ULONG , TransmitOffload : NL_INTERFACE_OFFLOAD_ROD , ReceiveOffload : NL_INTERFACE_OFFLOAD_ROD , DisableDefaultRoutes : BOOLEAN , } }
};
}
