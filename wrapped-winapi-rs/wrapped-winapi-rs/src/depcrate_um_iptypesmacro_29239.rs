// Generated macro for macro_29239 (macro)
macro_rules! Depcrate_um_iptypesmacro_29239 {
() => {
// Module: crate::um::iptypes
// Provides: {"macro_29239"}
// Dependencies: {}
STRUCT ! { struct IP_ADAPTER_INFO { Next : * mut IP_ADAPTER_INFO , ComboIndex : DWORD , AdapterName : [CHAR ; MAX_ADAPTER_NAME_LENGTH + 4] , Description : [CHAR ; MAX_ADAPTER_DESCRIPTION_LENGTH + 4] , AddressLength : UINT , Address : [BYTE ; MAX_ADAPTER_ADDRESS_LENGTH] , Index : DWORD , Type : UINT , DhcpEnabled : UINT , CurrentIpAddress : PIP_ADDR_STRING , IpAddressList : IP_ADDR_STRING , GatewayList : IP_ADDR_STRING , DhcpServer : IP_ADDR_STRING , HaveWins : BOOL , PrimaryWinsServer : IP_ADDR_STRING , SecondaryWinsServer : IP_ADDR_STRING , LeaseObtained : time_t , LeaseExpires : time_t , } }
};
}
