// Generated macro for impl_554 (impl)
macro_rules! Depcrate_retry_budget_tps_budgetimpl_554 {
() => {
// Module: crate::retry::budget::tps_budget
// Provides: {"impl_554"}
// Dependencies: {}
impl fmt :: Debug for TpsBudget { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Budget") . field ("deposit" , & self . deposit_amount) . field ("withdraw" , & self . withdraw_amount) . field ("balance" , & self . sum ()) . finish () } }
};
}
