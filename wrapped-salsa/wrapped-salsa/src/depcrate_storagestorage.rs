// Generated macro for Storage (struct)
macro_rules! Depcrate_storageStorage {
() => {
// Module: crate::storage
// Provides: {"Storage"}
// Dependencies: {}
# [doc = " Concrete implementation of the [`Database`] trait with local state that can be used to drive computations."] pub struct Storage < Db > { handle : StorageHandle < Db > , # [doc = " Per-thread state"] zalsa_local : zalsa_local :: ZalsaLocal , }
};
}
