// Generated macro for RemoveOnDrop (struct)
macro_rules! DepcrateRemoveOnDrop {
() => {
// Module: crate
// Provides: {"RemoveOnDrop"}
// Dependencies: {}
# [doc = " Drop guard for task removal. If a task panics, this makes sure"] # [doc = " it is removed from [`ActiveTasks`] properly."] struct RemoveOnDrop { id : task :: Id , storage : & 'static ActiveTasks , }
};
}
