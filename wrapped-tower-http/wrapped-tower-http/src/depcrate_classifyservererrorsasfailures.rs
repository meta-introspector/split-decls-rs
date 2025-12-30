// Generated macro for ServerErrorsAsFailures (struct)
macro_rules! Depcrate_classifyServerErrorsAsFailures {
() => {
// Module: crate::classify
// Provides: {"ServerErrorsAsFailures"}
// Dependencies: {}
# [doc = " The default classifier used for normal HTTP responses."] # [doc = ""] # [doc = " Responses with a `5xx` status code are considered failures, all others are considered"] # [doc = " successes."] # [derive (Clone , Debug , Default)] pub struct ServerErrorsAsFailures { _priv : () , }
};
}
