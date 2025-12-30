// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] impl From < AddressLoaderError > for TransactionError { fn from (err : AddressLoaderError) -> Self { match err { AddressLoaderError :: Disabled => Self :: UnsupportedVersion , AddressLoaderError :: SlotHashesSysvarNotFound => Self :: AccountNotFound , AddressLoaderError :: LookupTableAccountNotFound => Self :: AddressLookupTableNotFound , AddressLoaderError :: InvalidAccountOwner => Self :: InvalidAddressLookupTableOwner , AddressLoaderError :: InvalidAccountData => Self :: InvalidAddressLookupTableData , AddressLoaderError :: InvalidLookupIndex => Self :: InvalidAddressLookupTableIndex , } } }
};
}
