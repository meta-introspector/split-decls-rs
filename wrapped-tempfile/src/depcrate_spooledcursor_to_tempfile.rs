// Generated macro for cursor_to_tempfile (function)
macro_rules! Depcrate_spooledcursor_to_tempfile {
() => {
// Module: crate::spooled
// Provides: {"cursor_to_tempfile"}
// Dependencies: {}
# [doc = " Write a cursor into a temporary file, returning the temporary file."] fn cursor_to_tempfile (cursor : & Cursor < Vec < u8 > > , p : & Option < PathBuf >) -> io :: Result < File > { let mut file = match p { Some (p) => tempfile_in (p) ? , None => tempfile () ? , } ; file . write_all (cursor . get_ref ()) ? ; file . seek (SeekFrom :: Start (cursor . position ())) ? ; Ok (file) }
};
}
