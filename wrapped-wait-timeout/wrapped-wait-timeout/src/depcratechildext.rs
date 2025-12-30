// Generated macro for ChildExt (trait)
macro_rules! DepcrateChildExt {
() => {
// Module: crate
// Provides: {"ChildExt"}
// Dependencies: {}
# [doc = " Extension methods for the standard [`std::process::Child`] type."] pub trait ChildExt { # [doc = " Deprecated, use [`ChildExt::wait_timeout`] instead."] # [doc (hidden)] fn wait_timeout_ms (& mut self , ms : u32) -> io :: Result < Option < ExitStatus > > { self . wait_timeout (Duration :: from_millis (ms as u64)) } # [doc = " Wait for this child to exit, timing out after the duration `dur` has"] # [doc = " elapsed."] # [doc = ""] # [doc = " If `Ok(None)` is returned then the timeout period elapsed without the"] # [doc = " child exiting, and if `Ok(Some(..))` is returned then the child exited"] # [doc = " with the specified exit code."] fn wait_timeout (& mut self , dur : Duration) -> io :: Result < Option < ExitStatus > > ; }
};
}
