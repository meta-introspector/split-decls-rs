// Generated macro for impl_31 (impl)
macro_rules! Depcrate_archiveimpl_31 {
() => {
// Module: crate::archive
// Provides: {"impl_31"}
// Dependencies: {}
impl < R : Seek + Read > Archive < R > { # [doc = " Construct an iterator over the entries in this archive for a seekable"] # [doc = " reader. Seek will be used to efficiently skip over file contents."] # [doc = ""] # [doc = " Note that care must be taken to consider each entry within an archive in"] # [doc = " sequence. If entries are processed out of sequence (from what the"] # [doc = " iterator returns), then the contents read for each entry may be"] # [doc = " corrupted."] pub fn entries_with_seek (& mut self) -> io :: Result < Entries < '_ , R > > { let me : & Archive < dyn Read > = self ; let me_seekable : & Archive < dyn SeekRead > = self ; me . _entries (Some (me_seekable)) . map (| fields | Entries { fields , _ignored : marker :: PhantomData , }) } }
};
}
