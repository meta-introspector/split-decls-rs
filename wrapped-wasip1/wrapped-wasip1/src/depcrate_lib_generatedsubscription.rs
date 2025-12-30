// Generated macro for Subscription (struct)
macro_rules! Depcrate_lib_generatedSubscription {
() => {
// Module: crate::lib_generated
// Provides: {"Subscription"}
// Dependencies: {}
# [repr (C)] # [derive (Copy , Clone)] pub struct Subscription { # [doc = " User-provided value that is attached to the subscription in the"] # [doc = " implementation and returned through `event::userdata`."] pub userdata : Userdata , # [doc = " The type of the event to which to subscribe, and its contents"] pub u : SubscriptionU , }
};
}
