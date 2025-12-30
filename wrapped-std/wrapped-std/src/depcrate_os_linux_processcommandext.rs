// Generated macro for CommandExt (trait)
macro_rules! Depcrate_os_linux_processCommandExt {
() => {
// Module: crate::os::linux::process
// Provides: {"CommandExt"}
// Dependencies: {}
# [doc = " Os-specific extensions for [`Command`]"] # [doc = ""] # [doc = " [`Command`]: process::Command"] pub trait CommandExt : Sealed { # [doc = " Sets whether a [`PidFd`](struct@PidFd) should be created for the [`Child`]"] # [doc = " spawned by this [`Command`]."] # [doc = " By default, no pidfd will be created."] # [doc = ""] # [doc = " The pidfd can be retrieved from the child with [`pidfd`] or [`into_pidfd`]."] # [doc = ""] # [doc = " A pidfd will only be created if it is possible to do so"] # [doc = " in a guaranteed race-free manner. Otherwise, [`pidfd`] will return an error."] # [doc = ""] # [doc = " If a pidfd has been successfully created and not been taken from the `Child`"] # [doc = " then calls to `kill()`, `wait()` and `try_wait()` will use the pidfd"] # [doc = " instead of the pid. This can prevent pid recycling races, e.g."] # [doc = " those  caused by rogue libraries in the same process prematurely reaping"] # [doc = " zombie children via `waitpid(-1, ...)` calls."] # [doc = ""] # [doc = " [`Command`]: process::Command"] # [doc = " [`Child`]: process::Child"] # [doc = " [`pidfd`]: fn@ChildExt::pidfd"] # [doc = " [`into_pidfd`]: ChildExt::into_pidfd"] fn create_pidfd (& mut self , val : bool) -> & mut process :: Command ; }
};
}
