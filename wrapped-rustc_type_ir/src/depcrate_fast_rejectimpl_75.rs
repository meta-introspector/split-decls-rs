// Generated macro for impl_75 (impl)
macro_rules! Depcrate_fast_rejectimpl_75 {
() => {
// Module: crate::fast_reject
// Provides: {"impl_75"}
// Dependencies: {}
# [cfg (feature = "nightly")] impl < HCX : Clone , DefId : HashStable < HCX > > ToStableHashKey < HCX > for SimplifiedType < DefId > { type KeyType = Fingerprint ; # [inline] fn to_stable_hash_key (& self , hcx : & HCX) -> Fingerprint { let mut hasher = StableHasher :: new () ; let mut hcx : HCX = hcx . clone () ; self . hash_stable (& mut hcx , & mut hasher) ; hasher . finish () } }
};
}
