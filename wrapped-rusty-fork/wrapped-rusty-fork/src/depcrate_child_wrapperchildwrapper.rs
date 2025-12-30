// Generated macro for ChildWrapper (struct)
macro_rules! Depcrate_child_wrapperChildWrapper {
() => {
// Module: crate::child_wrapper
// Provides: {"ChildWrapper"}
// Dependencies: {}
# [doc = " Wraps a `std::process::Child` to coordinate state between `std` and"] # [doc = " `wait_timeout`."] # [doc = ""] # [doc = " This is necessary because the completion of a call to"] # [doc = " `wait_timeout::ChildExt::wait_timeout` leaves the `Child` in an"] # [doc = " inconsistent state, as it does not know the child has exited, and on Unix"] # [doc = " may end up referencing another process."] # [doc = ""] # [doc = " Documentation for this struct's methods is largely copied from the [Rust"] # [doc = " std docs](https://doc.rust-lang.org/stable/std/process/struct.Child.html)."] # [derive (Debug)] pub struct ChildWrapper { child : Child , exit_status : Option < ExitStatusWrapper > , }
};
}
