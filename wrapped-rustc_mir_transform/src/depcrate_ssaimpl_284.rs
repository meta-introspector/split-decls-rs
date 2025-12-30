// Generated macro for impl_284 (impl)
macro_rules! Depcrate_ssaimpl_284 {
() => {
// Module: crate::ssa
// Provides: {"impl_284"}
// Dependencies: {}
impl StorageLiveLocals { pub (crate) fn new (body : & Body < '_ > , always_storage_live_locals : & DenseBitSet < Local > ,) -> StorageLiveLocals { let mut storage_live = IndexVec :: from_elem (Set1 :: Empty , & body . local_decls) ; for local in always_storage_live_locals . iter () { storage_live [local] = Set1 :: One (DefLocation :: Argument) ; } for (block , bbdata) in body . basic_blocks . iter_enumerated () { for (statement_index , statement) in bbdata . statements . iter () . enumerate () { if let StatementKind :: StorageLive (local) = statement . kind { storage_live [local] . insert (DefLocation :: Assignment (Location { block , statement_index })) ; } } } debug ! (? storage_live) ; StorageLiveLocals { storage_live } } # [inline] pub (crate) fn has_single_storage (& self , local : Local) -> bool { matches ! (self . storage_live [local] , Set1 :: One (_)) } }
};
}
