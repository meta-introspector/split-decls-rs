// Generated macro for macro_5650 (macro)
macro_rules! Depcrate_shared_netioapimacro_5650 {
() => {
// Module: crate::shared::netioapi
// Provides: {"macro_5650"}
// Dependencies: {}
STRUCT ! { struct MIB_IF_ROW2 { InterfaceLuid : NET_LUID , InterfaceIndex : NET_IFINDEX , InterfaceGuid : GUID , Alias : [WCHAR ; IF_MAX_STRING_SIZE + 1] , Description : [WCHAR ; IF_MAX_STRING_SIZE + 1] , PhysicalAddressLength : ULONG , PhysicalAddress : [UCHAR ; IF_MAX_PHYS_ADDRESS_LENGTH] , PermanentPhysicalAddress : [UCHAR ; IF_MAX_PHYS_ADDRESS_LENGTH] , Mtu : ULONG , Type : IFTYPE , TunnelType : TUNNEL_TYPE , MediaType : NDIS_MEDIUM , PhysicalMediumType : NDIS_PHYSICAL_MEDIUM , AccessType : NET_IF_ACCESS_TYPE , DirectionType : NET_IF_DIRECTION_TYPE , InterfaceAndOperStatusFlags : MIB_IF_ROW2_InterfaceAndOperStatusFlags , OperStatus : IF_OPER_STATUS , AdminStatus : NET_IF_ADMIN_STATUS , MediaConnectState : NET_IF_MEDIA_CONNECT_STATE , NetworkGuid : NET_IF_NETWORK_GUID , ConnectionType : NET_IF_CONNECTION_TYPE , TransmitLinkSpeed : ULONG64 , ReceiveLinkSpeed : ULONG64 , InOctets : ULONG64 , InUcastPkts : ULONG64 , InNUcastPkts : ULONG64 , InDiscards : ULONG64 , InErrors : ULONG64 , InUnknownProtos : ULONG64 , InUcastOctets : ULONG64 , InMulticastOctets : ULONG64 , InBroadcastOctets : ULONG64 , OutOctets : ULONG64 , OutUcastPkts : ULONG64 , OutNUcastPkts : ULONG64 , OutDiscards : ULONG64 , OutErrors : ULONG64 , OutUcastOctets : ULONG64 , OutMulticastOctets : ULONG64 , OutBroadcastOctets : ULONG64 , OutQLen : ULONG64 , } }
};
}
