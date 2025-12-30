// Generated macro for CommandArgs (struct)
macro_rules! Depcrate_processCommandArgs {
() => {
// Module: crate::process
// Provides: {"CommandArgs"}
// Dependencies: {}
# [doc = " An iterator over the command arguments."] # [doc = ""] # [doc = " This struct is created by [`Command::get_args`]. See its documentation for"] # [doc = " more."] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "command_access" , since = "1.57.0")] # [derive (Debug)] pub struct CommandArgs < 'a > { inner : imp :: CommandArgs < 'a > , }
};
}
