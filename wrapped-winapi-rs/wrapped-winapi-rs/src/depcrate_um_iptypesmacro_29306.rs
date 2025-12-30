// Generated macro for macro_29306 (macro)
macro_rules! Depcrate_um_iptypesmacro_29306 {
() => {
// Module: crate::um::iptypes
// Provides: {"macro_29306"}
// Dependencies: {}
STRUCT ! { struct IP_ADAPTER_ADDRESSES_LH { u : IP_ADAPTER_ADDRESSES_LH_u , Next : * mut IP_ADAPTER_ADDRESSES_LH , AdapterName : PCHAR , FirstUnicastAddress : PIP_ADAPTER_UNICAST_ADDRESS_LH , FirstAnycastAddress : PIP_ADAPTER_ANYCAST_ADDRESS_XP , FirstMulticastAddress : PIP_ADAPTER_MULTICAST_ADDRESS_XP , FirstDnsServerAddress : PIP_ADAPTER_DNS_SERVER_ADDRESS_XP , DnsSuffix : PWCHAR , Description : PWCHAR , FriendlyName : PWCHAR , PhysicalAddress : [BYTE ; MAX_ADAPTER_ADDRESS_LENGTH] , PhysicalAddressLength : ULONG , Flags : ULONG , Mtu : ULONG , IfType : IFTYPE , OperStatus : IF_OPER_STATUS , Ipv6IfIndex : IF_INDEX , ZoneIndices : [ULONG ; 16] , FirstPrefix : PIP_ADAPTER_PREFIX_XP , TransmitLinkSpeed : ULONG64 , ReceiveLinkSpeed : ULONG64 , FirstWinsServerAddress : PIP_ADAPTER_WINS_SERVER_ADDRESS_LH , FirstGatewayAddress : PIP_ADAPTER_GATEWAY_ADDRESS_LH , Ipv4Metric : ULONG , Ipv6Metric : ULONG , Luid : IF_LUID , Dhcpv4Server : SOCKET_ADDRESS , CompartmentId : NET_IF_COMPARTMENT_ID , NetworkGuid : NET_IF_NETWORK_GUID , ConnectionType : NET_IF_CONNECTION_TYPE , TunnelType : TUNNEL_TYPE , Dhcpv6Server : SOCKET_ADDRESS , Dhcpv6ClientDuid : [BYTE ; MAX_DHCPV6_DUID_LENGTH] , Dhcpv6ClientDuidLength : ULONG , Dhcpv6Iaid : ULONG , FirstDnsSuffix : PIP_ADAPTER_DNS_SUFFIX , } }
};
}
