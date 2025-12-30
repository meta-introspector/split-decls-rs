// Generated macro for VSpace (struct)
macro_rules! Depcrate_hypervisor_vspaceVSpace {
() => {
// Module: crate::hypervisor::vspace
// Provides: {"VSpace"}
// Dependencies: {}
# [doc = " A VSpace allows to create and modify a (virtual) address space."] pub struct VSpace < 'a > { pub pml4 : & 'a mut PML4 , pmem : & 'a mut PhysicalMemory , }
};
}
