// Generated macro for Budget (trait)
macro_rules! Depcrate_retry_budgetBudget {
() => {
// Module: crate::retry::budget
// Provides: {"Budget"}
// Dependencies: {}
# [doc = " For more info about [`Budget`], please see the [module-level documentation]."] # [doc = ""] # [doc = " [module-level documentation]: self"] pub trait Budget { # [doc = " Store a \"deposit\" in the budget, which will be used to permit future"] # [doc = " withdrawals."] fn deposit (& self) ; # [doc = " Check whether there is enough \"balance\" in the budget to issue a new"] # [doc = " retry."] # [doc = ""] # [doc = " If there is not enough, false is returned."] fn withdraw (& self) -> bool ; }
};
}
