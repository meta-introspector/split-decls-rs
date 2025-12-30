// Generated macro for rusty_fork_id (macro)
macro_rules! Depcrate_sugarrusty_fork_id {
() => {
// Module: crate::sugar
// Provides: {"rusty_fork_id"}
// Dependencies: {}
# [doc = " Produce a hashable identifier unique to the particular macro invocation"] # [doc = " which is stable across processes of the same executable."] # [doc = ""] # [doc = " This is usually the best thing to pass for the `fork_id` argument of"] # [doc = " [`fork`](fn.fork.html)."] # [doc = ""] # [doc = " The type of the expression this macro expands to is"] # [doc = " [`RustyForkId`](struct.RustyForkId.html)."] # [macro_export] macro_rules ! rusty_fork_id { () => { { struct _RustyForkId ; $ crate :: RustyForkId :: of (:: std :: any :: TypeId :: of ::< _RustyForkId > ()) } } }
};
}
