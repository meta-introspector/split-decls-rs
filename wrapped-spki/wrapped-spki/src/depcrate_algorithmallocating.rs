// Generated macro for allocating (module)
macro_rules! Depcrate_algorithmallocating {
() => {
// Module: crate::algorithm
// Provides: {"allocating"}
// Dependencies: {}
# [cfg (feature = "alloc")] mod allocating { use super :: * ; use der :: referenced :: * ; impl < 'a > RefToOwned < 'a > for AlgorithmIdentifierRef < 'a > { type Owned = AlgorithmIdentifierOwned ; fn ref_to_owned (& self) -> Self :: Owned { AlgorithmIdentifier { oid : self . oid , parameters : self . parameters . ref_to_owned () , } } } impl OwnedToRef for AlgorithmIdentifierOwned { type Borrowed < 'a > = AlgorithmIdentifierRef < 'a > ; fn owned_to_ref (& self) -> Self :: Borrowed < '_ > { AlgorithmIdentifier { oid : self . oid , parameters : self . parameters . owned_to_ref () , } } } }
};
}
