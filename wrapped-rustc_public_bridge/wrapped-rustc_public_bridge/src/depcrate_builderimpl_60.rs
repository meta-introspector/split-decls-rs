// Generated macro for impl_60 (impl)
macro_rules! Depcrate_builderimpl_60 {
() => {
// Module: crate::builder
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'tcx > MutVisitor < 'tcx > for BodyBuilder < 'tcx > { fn visit_const_operand (& mut self , constant : & mut mir :: ConstOperand < 'tcx > , location : mir :: Location ,) { let const_ = constant . const_ ; let val = match const_ . eval (self . tcx , ty :: TypingEnv :: fully_monomorphized () , constant . span) { Ok (v) => v , Err (mir :: interpret :: ErrorHandled :: Reported (..)) => return , Err (mir :: interpret :: ErrorHandled :: TooGeneric (..)) => { unreachable ! ("Failed to evaluate instance constant: {:?}" , const_) } } ; let ty = constant . ty () ; constant . const_ = mir :: Const :: Val (val , ty) ; self . super_const_operand (constant , location) ; } fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } }
};
}
