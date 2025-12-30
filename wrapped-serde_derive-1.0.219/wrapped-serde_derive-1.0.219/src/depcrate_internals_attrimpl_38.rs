// Generated macro for impl_38 (impl)
macro_rules! Depcrate_internals_attrimpl_38 {
() => {
// Module: crate::internals::attr
// Provides: {"impl_38"}
// Dependencies: {}
impl < 'c > BoolAttr < 'c > { fn none (cx : & 'c Ctxt , name : Symbol) -> Self { BoolAttr (Attr :: none (cx , name)) } fn set_true < A : ToTokens > (& mut self , obj : A) { self . 0 . set (obj , ()) ; } fn get (& self) -> bool { self . 0 . value . is_some () } }
};
}
