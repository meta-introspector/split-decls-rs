// Generated macro for TpsBudget (struct)
macro_rules! Depcrate_retry_budget_tps_budgetTpsBudget {
() => {
// Module: crate::retry::budget::tps_budget
// Provides: {"TpsBudget"}
// Dependencies: {}
# [doc = " A Transactions Per Minute config for managing retry tokens."] # [doc = ""] # [doc = " [`TpsBudget`] uses a token bucket to decide if the request should be retried."] # [doc = ""] # [doc = " [`TpsBudget`] works by checking how much retries have been made in a certain period of time."] # [doc = " Minimum allowed number of retries are effectively reset on an interval. Allowed number of"] # [doc = " retries depends on failed request count in recent time frame."] # [doc = ""] # [doc = " For more info about [`Budget`], please see the [module-level documentation]."] # [doc = ""] # [doc = " [module-level documentation]: super"] pub struct TpsBudget { generation : Mutex < Generation > , # [doc = " Initial budget allowed for every second."] reserve : isize , # [doc = " Slots of a the TTL divided evenly."] slots : Box < [AtomicIsize] > , # [doc = " The amount of time represented by each slot."] window : Duration , # [doc = " The changers for the current slot to be committed"] # [doc = " after the slot expires."] writer : AtomicIsize , # [doc = " Amount of tokens to deposit for each put()."] deposit_amount : isize , # [doc = " Amount of tokens to withdraw for each try_get()."] withdraw_amount : isize , }
};
}
