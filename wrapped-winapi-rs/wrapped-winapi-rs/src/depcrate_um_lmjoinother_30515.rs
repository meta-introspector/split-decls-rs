// Generated macro for other_30515 (other)
macro_rules! Depcrate_um_lmjoinother_30515 {
() => {
// Module: crate::um::lmjoin
// Provides: {"other_30515"}
// Dependencies: {}
extern "system" { pub fn NetAddAlternateComputerName (Server : LPCWSTR , AlternateName : LPCWSTR , DomainAccount : LPCWSTR , DomainAccountPassword : LPCWSTR , Reserved : ULONG ,) -> NET_API_STATUS ; pub fn NetRemoveAlternateComputerName (Server : LPCWSTR , AlternateName : LPCWSTR , DomainAccount : LPCWSTR , DomainAccountPassword : LPCWSTR , Reserved : ULONG ,) -> NET_API_STATUS ; pub fn NetSetPrimaryComputerName (Server : LPCWSTR , PrimaryName : LPCWSTR , DomainAccount : LPCWSTR , DomainAccountPassword : LPCWSTR , Reserved : ULONG ,) -> NET_API_STATUS ; }
};
}
