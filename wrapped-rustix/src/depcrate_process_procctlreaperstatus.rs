// Generated macro for ReaperStatus (struct)
macro_rules! Depcrate_process_procctlReaperStatus {
() => {
// Module: crate::process::procctl
// Provides: {"ReaperStatus"}
// Dependencies: {}
# [doc = " Reaper status as returned by [`get_reaper_status`]."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct ReaperStatus { # [doc = " The flags."] pub flags : ReaperStatusFlags , # [doc = " The number of children of the reaper among the descendants."] pub children : usize , # [doc = " The total number of descendants of the reaper(s), not counting"] # [doc = " descendants of the reaper in the subtree."] pub descendants : usize , # [doc = " The pid of the reaper for the specified process id."] pub reaper : Pid , # [doc = " The pid of one reaper child if there are any descendants."] pub pid : Option < Pid > , }
};
}
