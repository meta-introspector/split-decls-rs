// Generated macro for impl_37 (impl)
macro_rules! Depcrate_archiveimpl_37 {
() => {
// Module: crate::archive
// Provides: {"impl_37"}
// Dependencies: {}
impl < R : ? Sized + Read > Read for & ArchiveInner < R > { fn read (& mut self , into : & mut [u8]) -> io :: Result < usize > { let i = self . obj . borrow_mut () . read (into) ? ; self . pos . set (self . pos . get () + i as u64) ; Ok (i) } }
};
}
