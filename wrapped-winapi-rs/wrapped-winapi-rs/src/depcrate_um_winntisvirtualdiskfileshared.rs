// Generated macro for IsVirtualDiskFileShared (function)
macro_rules! Depcrate_um_winntIsVirtualDiskFileShared {
() => {
// Module: crate::um::winnt
// Provides: {"IsVirtualDiskFileShared"}
// Dependencies: {}
# [inline] pub fn IsVirtualDiskFileShared (HandleState : SharedVirtualDiskHandleState) -> bool { (HandleState & SharedVirtualDiskHandleStateFileShared) != 0 }
};
}
