// Generated macro for impl_49 (impl)
macro_rules! Depcrate_collectorimpl_49 {
() => {
// Module: crate::collector
// Provides: {"impl_49"}
// Dependencies: {}
impl < 'a , 'tcx > MirUsedCollector < 'a , 'tcx > { fn monomorphize < T > (& self , value : T) -> T where T : TypeFoldable < TyCtxt < 'tcx > > , { trace ! ("monomorphize: self.instance={:?}" , self . instance) ; self . instance . instantiate_mir_and_normalize_erasing_regions (self . tcx , ty :: TypingEnv :: fully_monomorphized () , ty :: EarlyBinder :: bind (value) ,) } # [doc = " Evaluates a *not yet monomorphized* constant."] fn eval_constant (& mut self , constant : & mir :: ConstOperand < 'tcx >) -> Option < mir :: ConstValue > { let const_ = self . monomorphize (constant . const_) ; match const_ . eval (self . tcx , ty :: TypingEnv :: fully_monomorphized () , constant . span) { Ok (v) => Some (v) , Err (ErrorHandled :: TooGeneric (..)) => span_bug ! (constant . span , "collection encountered polymorphic constant: {:?}" , const_) , Err (err @ ErrorHandled :: Reported (..)) => { err . emit_note (self . tcx) ; return None ; } } } }
};
}
