// Generated macro for impl_672 (impl)
macro_rules! Depcrate_importsimpl_672 {
() => {
// Module: crate::imports
// Provides: {"impl_672"}
// Dependencies: {}
impl Ord for UseTree { fn cmp (& self , other : & UseTree) -> Ordering { for (a , b) in self . path . iter () . zip (other . path . iter ()) { let ord = a . cmp (b) ; if ord != Ordering :: Equal && a . remove_alias () . cmp (& b . remove_alias ()) != Ordering :: Equal { return ord ; } } self . path . len () . cmp (& other . path . len ()) } }
};
}
