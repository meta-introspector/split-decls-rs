// Generated macro for impl_1664 (impl)
macro_rules! Depcrate_os_unix_processimpl_1664 {
() => {
// Module: crate::os::unix::process
// Provides: {"impl_1664"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl CommandExt for process :: Command { fn uid (& mut self , id : UserId) -> & mut process :: Command { self . as_inner_mut () . uid (id) ; self } fn gid (& mut self , id : GroupId) -> & mut process :: Command { self . as_inner_mut () . gid (id) ; self } fn groups (& mut self , groups : & [GroupId]) -> & mut process :: Command { self . as_inner_mut () . groups (groups) ; self } unsafe fn pre_exec < F > (& mut self , f : F) -> & mut process :: Command where F : FnMut () -> io :: Result < () > + Send + Sync + 'static , { self . as_inner_mut () . pre_exec (Box :: new (f)) ; self } fn exec (& mut self) -> io :: Error { self . as_inner_mut () . exec (sys :: process :: Stdio :: Inherit) } fn arg0 < S > (& mut self , arg : S) -> & mut process :: Command where S : AsRef < OsStr > , { self . as_inner_mut () . set_arg_0 (arg . as_ref ()) ; self } fn process_group (& mut self , pgroup : i32) -> & mut process :: Command { self . as_inner_mut () . pgroup (pgroup) ; self } fn chroot < P : AsRef < Path > > (& mut self , dir : P) -> & mut process :: Command { self . as_inner_mut () . chroot (dir . as_ref ()) ; self } fn setsid (& mut self , setsid : bool) -> & mut process :: Command { self . as_inner_mut () . setsid (setsid) ; self } }
};
}
