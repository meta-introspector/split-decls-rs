// Generated macro for impl_273 (impl)
macro_rules! Depcrate_zerotrieimpl_273 {
() => {
// Module: crate::zerotrie
// Provides: {"impl_273"}
// Dependencies: {}
# [cfg (feature = "zerofrom")] impl < 'zf , Store1 , Store2 > zerofrom :: ZeroFrom < 'zf , ZeroTrie < Store1 > > for ZeroTrie < Store2 > where Store2 : zerofrom :: ZeroFrom < 'zf , Store1 > , { fn zero_from (other : & 'zf ZeroTrie < Store1 >) -> Self { use zerofrom :: ZeroFrom ; impl_dispatch ! (& other , ZeroFrom :: zero_from ()) } }
};
}
