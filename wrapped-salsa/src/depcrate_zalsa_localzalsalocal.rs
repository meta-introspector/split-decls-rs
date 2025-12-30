// Generated macro for ZalsaLocal (struct)
macro_rules! Depcrate_zalsa_localZalsaLocal {
() => {
// Module: crate::zalsa_local
// Provides: {"ZalsaLocal"}
// Dependencies: {}
# [doc = " State that is specific to a single execution thread."] # [doc = ""] # [doc = " Internally, this type uses ref-cells."] # [doc = ""] # [doc = " **Note also that all mutations to the database handle (and hence"] # [doc = " to the local-state) must be undone during unwinding.**"] pub struct ZalsaLocal { # [doc = " Vector of active queries."] # [doc = ""] # [doc = " Unwinding note: pushes onto this vector must be popped -- even"] # [doc = " during unwinding."] query_stack : RefCell < QueryStack > , # [doc = " Stores the most recent page for a given ingredient."] # [doc = " This is thread-local to avoid contention."] most_recent_pages : UnsafeCell < FxHashMap < IngredientIndex , PageIndex > > , }
};
}
