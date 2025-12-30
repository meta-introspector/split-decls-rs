// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] impl fmt :: Display for AddressLoaderError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { Self :: Disabled => f . write_str ("Address loading from lookup tables is disabled") , Self :: SlotHashesSysvarNotFound => f . write_str ("Failed to load slot hashes sysvar") , Self :: LookupTableAccountNotFound => { f . write_str ("Attempted to lookup addresses from a table that does not exist") } Self :: InvalidAccountOwner => f . write_str ("Attempted to lookup addresses from an account owned by the wrong program" ,) , Self :: InvalidAccountData => { f . write_str ("Attempted to lookup addresses from an invalid account") } Self :: InvalidLookupIndex => f . write_str ("Address lookup contains an invalid index") , } } }
};
}
