// Generated macro for SubscriptionClock (struct)
macro_rules! Depcrate_lib_generatedSubscriptionClock {
() => {
// Module: crate::lib_generated
// Provides: {"SubscriptionClock"}
// Dependencies: {}
# [repr (C)] # [derive (Copy , Clone , Debug)] pub struct SubscriptionClock { # [doc = " The clock against which to compare the timestamp."] pub id : Clockid , # [doc = " The absolute or relative timestamp."] pub timeout : Timestamp , # [doc = " The amount of time that the implementation may wait additionally"] # [doc = " to coalesce with other events."] pub precision : Timestamp , # [doc = " Flags specifying whether the timeout is absolute or relative"] pub flags : Subclockflags , }
};
}
