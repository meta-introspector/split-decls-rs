macro_rules! cursor_to_tempfile {
    () => {
        # [doc = " Write a cursor into a temporary file, returning the temporary file."] fn cursor_to_tempfile (cursor : & Cursor < Vec < u8 > > , p : & Option < PathBuf >) -> io :: Result < File > { let mut file = match p { Some (p) => tempfile_in (p) ? , None => tempfile () ? , } ; file . write_all (cursor . get_ref ()) ? ; file . seek (SeekFrom :: Start (cursor . position ())) ? ; Ok (file) }
    };
}

cursor_to_tempfile!()