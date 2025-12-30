// Generated macro for impl_2262 (impl)
macro_rules! Depcrate_fs_raw_dirimpl_2262 {
() => {
// Module: crate::fs::raw_dir
// Provides: {"impl_2262"}
// Dependencies: {}
impl < 'buf , Fd : AsFd > RawDir < 'buf , Fd > { # [doc = " Identical to [`Iterator::next`] except that [`Iterator::Item`] borrows"] # [doc = " from self."] # [doc = ""] # [doc = " Note: this interface will be broken to implement a stdlib iterator API"] # [doc = " with GAT support once one becomes available."] # [allow (unsafe_code)] # [allow (clippy :: should_implement_trait)] pub fn next (& mut self) -> Option < io :: Result < RawDirEntry < '_ > > > { if self . is_buffer_empty () { match getdents_uninit (self . fd . as_fd () , self . buf) { Ok (0) => return None , Ok (bytes_read) => { self . initialized = bytes_read ; self . offset = 0 ; } Err (e) => return Some (Err (e)) , } } let dirent_ptr = self . buf [self . offset ..] . as_ptr () ; let dirent = unsafe { & * dirent_ptr . cast :: < linux_dirent64 > () } ; self . offset += usize :: from (dirent . d_reclen) ; Some (Ok (RawDirEntry { file_type : dirent . d_type , inode_number : dirent . d_ino . into () , next_entry_cookie : dirent . d_off . into () , file_name : unsafe { CStr :: from_ptr (dirent . d_name . as_ptr () . cast ()) } , })) } # [doc = " Returns true if the internal buffer is empty and will be refilled when"] # [doc = " calling [`next`]."] # [doc = ""] # [doc = " [`next`]: Self::next"] pub fn is_buffer_empty (& self) -> bool { self . offset >= self . initialized } }
};
}
