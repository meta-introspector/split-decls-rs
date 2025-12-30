// Generated macro for dump_table (function)
macro_rules! Depcrate_hypervisor_vspacedump_table {
() => {
// Module: crate::hypervisor::vspace
// Provides: {"dump_table"}
// Dependencies: {}
# [allow (unused)] pub unsafe fn dump_table (pml4_table : & PML4) { for (pml_idx , pml_item) in pml4_table . iter () . enumerate () { if pml_item . is_present () { let pdpt_table = transmute :: < VAddr , & mut PDPT > (VAddr :: from_u64 (pml_item . address () . as_u64 ())) ; for (pdpt_idx , pdpt_item) in pdpt_table . iter () . enumerate () { if pdpt_item . is_present () { let pd_table = transmute :: < VAddr , & mut PD > (VAddr :: from_u64 (pdpt_item . address () . as_u64 ())) ; if pdpt_item . is_page () { let vaddr : usize = (512 * (512 * (512 * 0x1000))) * pml_idx + (512 * (512 * 0x1000)) * pdpt_idx ; info ! ("PDPT item: vaddr 0x{:x} maps to {:?}" , vaddr , pdpt_item) ; } else { for (pd_idx , pd_item) in pd_table . iter () . enumerate () { if pd_item . is_present () { let ptes = transmute :: < VAddr , & mut PT > (VAddr :: from_u64 (pd_item . address () . as_u64 () ,)) ; if pd_item . is_page () { let vaddr : usize = (512 * (512 * (512 * 0x1000))) * pml_idx + (512 * (512 * 0x1000)) * pdpt_idx + (512 * 0x1000) * pd_idx ; info ! ("PD item: vaddr 0x{:x} maps to {:?}" , vaddr , pd_item) ; } else { assert ! (! pd_item . is_page ()) ; for (pte_idx , pte) in ptes . iter () . enumerate () { let vaddr : usize = (512 * (512 * (512 * 0x1000))) * pml_idx + (512 * (512 * 0x1000)) * pdpt_idx + (512 * 0x1000) * pd_idx + (0x1000) * pte_idx ; if pte . is_present () { info ! ("PT item: vaddr 0x{:x} maps to flags {:?}" , vaddr , pte) ; } } } } } } } } } } }
};
}
