// Generated macro for Optional (struct)
macro_rules! Depcrate_util_optionalOptional {
() => {
// Module: crate::util::optional
// Provides: {"Optional"}
// Dependencies: {}
# [doc = " Optionally forwards requests to an inner service."] # [doc = ""] # [doc = " If the inner service is [`None`], [`optional::None`] is returned as the response."] # [doc = ""] # [doc = " [`optional::None`]: crate::util::error::optional::None"] # [derive (Debug)] pub struct Optional < T > { inner : Option < T > , }
};
}
