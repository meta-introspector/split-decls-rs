// Generated macro for test (module)
macro_rules! Depcrate_sparse_chunk_refpooltest {
() => {
// Module: crate::sparse_chunk::refpool
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use :: refpool :: { Pool , PoolRef } ; # [test] fn default_and_clone () { let pool : Pool < SparseChunk < usize , 64 > > = Pool :: new (16) ; let mut ref1 = PoolRef :: default (& pool) ; { let chunk = PoolRef :: make_mut (& pool , & mut ref1) ; chunk . insert (5 , 13) ; chunk . insert (10 , 37) ; chunk . insert (31 , 337) ; } let ref2 = PoolRef :: cloned (& pool , & ref1) ; assert_eq ! (ref1 , ref2) ; assert ! (! PoolRef :: ptr_eq (& ref1 , & ref2)) ; } }
};
}
