// Generated macro for AddressLoaderError (enum)
macro_rules! DepcrateAddressLoaderError {
() => {
// Module: crate
// Provides: {"AddressLoaderError"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] # [derive (Debug , PartialEq , Eq , Clone)] pub enum AddressLoaderError { # [doc = " Address loading from lookup tables is disabled"] Disabled , # [doc = " Failed to load slot hashes sysvar"] SlotHashesSysvarNotFound , # [doc = " Attempted to lookup addresses from a table that does not exist"] LookupTableAccountNotFound , # [doc = " Attempted to lookup addresses from an account owned by the wrong program"] InvalidAccountOwner , # [doc = " Attempted to lookup addresses from an invalid account"] InvalidAccountData , # [doc = " Address lookup contains an invalid index"] InvalidLookupIndex , }
};
}
