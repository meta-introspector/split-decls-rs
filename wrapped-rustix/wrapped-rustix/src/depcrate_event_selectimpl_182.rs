// Generated macro for impl_182 (impl)
macro_rules! Depcrate_event_selectimpl_182 {
() => {
// Module: crate::event::select
// Provides: {"impl_182"}
// Dependencies: {}
# [cfg (not (any (windows , target_os = "wasi")))] impl < 'a > Iterator for FdSetIter < 'a > { type Item = RawFd ; fn next (& mut self) -> Option < Self :: Item > { if let Some (element) = self . fds . get (self . current as usize / BITS) { let shifted = element . 0 >> ((self . current as usize % BITS) as u32) ; if shifted != 0 { let fd = self . current + shifted . trailing_zeros () as RawFd ; self . current = fd + 1 ; return Some (fd) ; } if let Some (index) = self . fds [(self . current as usize / BITS) + 1 ..] . iter () . position (| element | element . 0 != 0) { let index = index + (self . current as usize / BITS) + 1 ; let element = self . fds [index] . 0 ; let fd = (index * BITS) as RawFd + element . trailing_zeros () as RawFd ; self . current = fd + 1 ; return Some (fd) ; } } None } }
};
}
