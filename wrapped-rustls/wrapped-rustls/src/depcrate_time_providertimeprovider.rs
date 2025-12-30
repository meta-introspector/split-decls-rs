// Generated macro for TimeProvider (trait)
macro_rules! Depcrate_time_providerTimeProvider {
() => {
// Module: crate::time_provider
// Provides: {"TimeProvider"}
// Dependencies: {}
# [doc = " An object that provides the current time."] # [doc = ""] # [doc = " This is used to, for example, check if a certificate has expired during"] # [doc = " certificate validation, or to check the age of a ticket."] pub trait TimeProvider : Debug + Send + Sync { # [doc = " Returns the current wall time."] # [doc = ""] # [doc = " This is not required to be monotonic."] # [doc = ""] # [doc = " Return `None` if unable to retrieve the time."] fn current_time (& self) -> Option < UnixTime > ; }
};
}
