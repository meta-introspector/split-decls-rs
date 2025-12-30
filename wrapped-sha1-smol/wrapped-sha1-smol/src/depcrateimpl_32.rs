// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl Blocks { fn input < F > (& mut self , mut input : & [u8] , mut f : F) where F : FnMut (& [u8 ; 64]) , { if self . len > 0 { let len = self . len as usize ; let amt = cmp :: min (input . len () , self . block . len () - len) ; self . block [len .. len + amt] . clone_from_slice (& input [.. amt]) ; if len + amt == self . block . len () { f (& self . block) ; self . len = 0 ; input = & input [amt ..] ; } else { self . len += amt as u32 ; return ; } } assert_eq ! (self . len , 0) ; for chunk in input . chunks (64) { if chunk . len () == 64 { f (as_block (chunk)) } else { self . block [.. chunk . len ()] . clone_from_slice (chunk) ; self . len = chunk . len () as u32 ; } } } }
};
}
