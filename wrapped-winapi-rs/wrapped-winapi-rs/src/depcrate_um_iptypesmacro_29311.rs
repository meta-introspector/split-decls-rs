// Generated macro for macro_29311 (macro)
macro_rules! Depcrate_um_iptypesmacro_29311 {
() => {
// Module: crate::um::iptypes
// Provides: {"macro_29311"}
// Dependencies: {}
STRUCT ! { struct IP_ADAPTER_ADDRESSES_XP { u : IP_ADAPTER_ADDRESSES_XP_u , Next : * mut IP_ADAPTER_ADDRESSES_XP , AdapterName : PCHAR , FirstUnicastAddress : PIP_ADAPTER_UNICAST_ADDRESS_XP , FirstAnycastAddress : PIP_ADAPTER_ANYCAST_ADDRESS_XP , FirstMulticastAddress : PIP_ADAPTER_MULTICAST_ADDRESS_XP , FirstDnsServerAddress : PIP_ADAPTER_DNS_SERVER_ADDRESS_XP , DnsSuffix : PWCHAR , Description : PWCHAR , FriendlyName : PWCHAR , PhysicalAddress : [BYTE ; MAX_ADAPTER_ADDRESS_LENGTH] , PhysicalAddressLength : DWORD , Flags : DWORD , Mtu : DWORD , IfType : DWORD , OperStatus : IF_OPER_STATUS , Ipv6IfIndex : DWORD , ZoneIndices : [DWORD ; 16] , FirstPrefix : PIP_ADAPTER_PREFIX_XP , } }
};
}
