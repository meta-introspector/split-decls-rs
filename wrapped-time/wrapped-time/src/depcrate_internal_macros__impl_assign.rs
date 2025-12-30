// Generated macro for __impl_assign (macro)
macro_rules! Depcrate_internal_macros__impl_assign {
() => {
// Module: crate::internal_macros
// Provides: {"__impl_assign"}
// Dependencies: {}
# [doc = " Helper macro for easily implementing `OpAssign`."] macro_rules ! __impl_assign { ($ sym : tt $ op : ident $ fn : ident $ target : ty : $ ($ (# [$ attr : meta]) * $ t : ty) ,+) => { $ (# [allow (unused_qualifications)] $ (# [$ attr]) * impl core :: ops ::$ op <$ t > for $ target { # [inline] fn $ fn (& mut self , rhs : $ t) { * self = * self $ sym rhs ; } }) + } ; }
};
}
