// Generated macro for paddr_to_vaddr (function)
macro_rules! Depcrate_hypervisor_vspacepaddr_to_vaddr {
() => {
// Module: crate::hypervisor::vspace
// Provides: {"paddr_to_vaddr"}
// Dependencies: {}
fn paddr_to_vaddr (p : PAddr) -> VAddr { VAddr (p . into ()) }
};
}
