// Generated macro for impl_32 (impl)
macro_rules! Depcrate_archiveimpl_32 {
() => {
// Module: crate::archive
// Provides: {"impl_32"}
// Dependencies: {}
impl Archive < dyn Read + '_ > { fn _entries < 'a > (& 'a self , seekable_archive : Option < & 'a Archive < dyn SeekRead + 'a > > ,) -> io :: Result < EntriesFields < 'a > > { if self . inner . pos . get () != 0 { return Err (other ("cannot call entries unless archive is at \
                 position 0" ,)) ; } Ok (EntriesFields { archive : self , seekable_archive , done : false , next : 0 , raw : false , }) } fn _unpack (& mut self , dst : & Path) -> io :: Result < () > { if dst . symlink_metadata () . is_err () { fs :: create_dir_all (dst) . map_err (| e | TarError :: new (format ! ("failed to create `{}`" , dst . display ()) , e)) ? ; } let dst = & dst . canonicalize () . unwrap_or (dst . to_path_buf ()) ; let mut directories = Vec :: new () ; for entry in self . _entries (None) ? { let mut file = entry . map_err (| e | TarError :: new ("failed to iterate over archive" , e)) ? ; if file . header () . entry_type () == crate :: EntryType :: Directory { directories . push (file) ; } else { file . unpack_in (dst) ? ; } } directories . sort_by (| a , b | b . path_bytes () . cmp (& a . path_bytes ())) ; for mut dir in directories { dir . unpack_in (dst) ? ; } Ok (()) } }
};
}
