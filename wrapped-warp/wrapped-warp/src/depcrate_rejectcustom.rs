// Generated macro for custom (function)
macro_rules! Depcrate_rejectcustom {
() => {
// Module: crate::reject
// Provides: {"custom"}
// Dependencies: {}
# [doc = " Rejects a request with a custom cause."] # [doc = ""] # [doc = " A [`recover`][] filter should convert this `Rejection` into a `Reply`,"] # [doc = " or else this will be returned as a `500 Internal Server Error`."] # [doc = ""] # [doc = " [`recover`]: ../trait.Filter.html#method.recover"] pub fn custom < T : Reject > (err : T) -> Rejection { Rejection :: custom (Box :: new (err)) }
};
}
