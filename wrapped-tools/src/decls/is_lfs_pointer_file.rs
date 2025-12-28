macro_rules! is_lfs_pointer_file {
    () => {
        fn is_lfs_pointer_file (path : & Path) -> bool { const PREFIX : & [u8] = b"version https://git-lfs" ; let mut buf = [0_u8 ; PREFIX . len ()] ; std :: fs :: OpenOptions :: new () . read (true) . open (path) . is_ok_and (| mut f | f . read_exact (& mut buf) . is_ok_and (| _ | buf . starts_with (PREFIX))) }
    };
}

is_lfs_pointer_file!();