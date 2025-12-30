// Generated macro for impl_552 (impl)
macro_rules! Depcrate_retry_budget_tps_budgetimpl_552 {
() => {
// Module: crate::retry::budget::tps_budget
// Provides: {"impl_552"}
// Dependencies: {}
impl Budget for TpsBudget { fn deposit (& self) { self . put (self . deposit_amount) } fn withdraw (& self) -> bool { self . try_get (self . withdraw_amount) } }
};
}
