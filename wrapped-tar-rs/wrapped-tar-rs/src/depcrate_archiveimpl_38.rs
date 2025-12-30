// Generated macro for impl_38 (impl)
macro_rules! Depcrate_archiveimpl_38 {
() => {
// Module: crate::archive
// Provides: {"impl_38"}
// Dependencies: {}
impl < R : ? Sized + Seek > Seek for & ArchiveInner < R > { fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { let pos = self . obj . borrow_mut () . seek (pos) ? ; self . pos . set (pos) ; Ok (pos) } }
};
}
