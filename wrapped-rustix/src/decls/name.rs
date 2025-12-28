macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! name {
    () => {
        deps!();
        # [doc = " Get the name of the calling thread."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_GET_NAME,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_GET_NAME,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [cfg (feature = "alloc")] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] pub fn name () -> io :: Result < CString > { let mut buffer = [0_u8 ; 16] ; unsafe { prctl_2args (PR_GET_NAME , buffer . as_mut_ptr () . cast ()) ? } ; let len = buffer . iter () . position (| & x | x == 0_u8) . unwrap_or (0) ; CString :: new (& buffer [.. len]) . map_err (| _r | io :: Errno :: ILSEQ) }
    };
}

name!();