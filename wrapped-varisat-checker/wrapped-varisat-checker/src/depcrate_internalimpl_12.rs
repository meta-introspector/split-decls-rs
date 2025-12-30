// Generated macro for impl_12 (impl)
macro_rules! Depcrate_internalimpl_12 {
() => {
// Module: crate::internal
// Provides: {"impl_12"}
// Dependencies: {}
impl < 'a > SelfChecker for Checker < 'a > { fn self_check_step (& mut self , step : ProofStep) -> Result < () , CheckerError > { self . ctx . checker_state . step += 1 ; let mut ctx = self . ctx . into_partial_ref_mut () ; check_step (ctx . borrow () , step) } fn self_check_delayed_steps (& mut self) -> Result < () , CheckerError > { let mut ctx = self . ctx . into_partial_ref_mut () ; process_unit_conflicts (ctx . borrow ()) } }
};
}
