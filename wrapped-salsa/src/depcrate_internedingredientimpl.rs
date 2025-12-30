// Generated macro for IngredientImpl (struct)
macro_rules! Depcrate_internedIngredientImpl {
() => {
// Module: crate::interned
// Provides: {"IngredientImpl"}
// Dependencies: {}
# [doc = " The interned ingredient hashes values of type `C::Fields` to produce an `Id`."] # [doc = ""] # [doc = " It used to store interned structs but also to store the ID fields of a tracked struct."] # [doc = " Interned values are garbage collected and their memory reused based on an LRU heuristic."] pub struct IngredientImpl < C : Configuration > { # [doc = " Index of this ingredient in the database (used to construct database-IDs, etc)."] ingredient_index : IngredientIndex , # [doc = " A hasher for the sharded ID maps."] hasher : FxBuildHasher , # [doc = " A shift used to determine the shard for a given hash."] shift : u32 , # [doc = " Sharded data that can only be accessed through a lock."] shards : Box < [CachePadded < Mutex < IngredientShard < C > > >] > , # [doc = " A queue of recent revisions in which values were interned."] revision_queue : RevisionQueue < C > , memo_table_types : Arc < MemoTableTypes > , _marker : PhantomData < fn () -> C > , }
};
}
