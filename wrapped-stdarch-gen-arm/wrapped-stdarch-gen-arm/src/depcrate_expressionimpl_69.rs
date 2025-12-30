// Generated macro for impl_69 (impl)
macro_rules! Depcrate_expressionimpl_69 {
() => {
// Module: crate::expression
// Provides: {"impl_69"}
// Dependencies: {}
impl FnCall { pub fn new_expression (fn_ptr : Expression , arguments : Vec < Expression >) -> Expression { FnCall (Box :: new (fn_ptr) , arguments , Vec :: new () , false) . into () } pub fn new_unsafe_expression (fn_ptr : Expression , arguments : Vec < Expression >) -> Expression { FnCall (Box :: new (fn_ptr) , arguments , Vec :: new () , true) . into () } pub fn is_llvm_link_call (& self , llvm_link_name : & str) -> bool { self . is_expected_call (llvm_link_name) } pub fn is_target_feature_call (& self) -> bool { self . is_expected_call ("target_feature") } pub fn is_expected_call (& self , fn_call_name : & str) -> bool { if let Expression :: Identifier (fn_name , IdentifierType :: Symbol) = self . 0 . as_ref () { fn_name . to_string () == fn_call_name } else { false } } pub fn pre_build (& mut self , ctx : & mut Context) -> context :: Result { self . 0 . pre_build (ctx) ? ; self . 1 . iter_mut () . chain (self . 2 . iter_mut ()) . try_for_each (| ex | ex . pre_build (ctx)) } pub fn build (& mut self , intrinsic : & Intrinsic , ctx : & mut Context) -> context :: Result { self . 0 . build (intrinsic , ctx) ? ; self . 1 . iter_mut () . chain (self . 2 . iter_mut ()) . try_for_each (| ex | ex . build (intrinsic , ctx)) } }
};
}
