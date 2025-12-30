// Generated macro for InitGuard (struct)
macro_rules! Depcrate_page_slotInitGuard {
() => {
// Module: crate::page::slot
// Provides: {"InitGuard"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct InitGuard < T , C : cfg :: Config = cfg :: DefaultConfig > { slot : ptr :: NonNull < Slot < T , C > > , curr_lifecycle : usize , released : bool , }
};
}
