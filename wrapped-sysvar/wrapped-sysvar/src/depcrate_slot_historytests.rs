// Generated macro for tests (module)
macro_rules! Depcrate_slot_historytests {
() => {
// Module: crate::slot_history
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_size_of () { assert_eq ! (SlotHistory :: size_of () , bincode :: serialized_size (& SlotHistory :: default ()) . unwrap () as usize) ; } }
};
}
