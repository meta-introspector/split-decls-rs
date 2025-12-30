// Generated macro for impl_73 (impl)
macro_rules! Depcrate_cost_checkerimpl_73 {
() => {
// Module: crate::cost_checker
// Provides: {"impl_73"}
// Dependencies: {}
impl < 'b , 'tcx > CostChecker < 'b , 'tcx > { pub (super) fn new (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , instance : Option < ty :: Instance < 'tcx > > , callee_body : & 'b Body < 'tcx > ,) -> CostChecker < 'b , 'tcx > { CostChecker { tcx , typing_env , callee_body , instance , penalty : 0 , bonus : 0 } } # [doc = " Add function-level costs not well-represented by the block-level costs."] # [doc = ""] # [doc = " Needed because the `CostChecker` is used sometimes for just blocks,"] # [doc = " and even the full `Inline` doesn't call `visit_body`, so there's nowhere"] # [doc = " to put this logic in the visitor."] pub (super) fn add_function_level_costs (& mut self) { if self . callee_body . basic_blocks . iter () . filter (| bbd | is_call_like (bbd . terminator ())) . count () == 1 { self . bonus += CALL_PENALTY ; } } pub (super) fn cost (& self) -> usize { usize :: saturating_sub (self . penalty , self . bonus) } fn instantiate_ty (& self , v : Ty < 'tcx >) -> Ty < 'tcx > { if let Some (instance) = self . instance { instance . instantiate_mir (self . tcx , ty :: EarlyBinder :: bind (& v)) } else { v } } }
};
}
