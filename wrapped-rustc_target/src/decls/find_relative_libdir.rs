macro_rules! find_relative_libdir {
    () => {
        # [doc = " The name of the directory rustc expects libraries to be located."] fn find_relative_libdir (sysroot : & Path) -> std :: borrow :: Cow < 'static , str > { # [cfg (target_pointer_width = "64")] const PRIMARY_LIB_DIR : & str = "lib64" ; # [cfg (target_pointer_width = "32")] const PRIMARY_LIB_DIR : & str = "lib32" ; const SECONDARY_LIB_DIR : & str = "lib" ; match option_env ! ("CFG_LIBDIR_RELATIVE") { None | Some ("lib") => { if sysroot . join (PRIMARY_LIB_DIR) . join (RUST_LIB_DIR) . exists () { PRIMARY_LIB_DIR . into () } else { SECONDARY_LIB_DIR . into () } } Some (libdir) => libdir . into () , } }
    };
}

find_relative_libdir!()