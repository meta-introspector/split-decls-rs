macro_rules! deps {
    () => {
        Error!();
        Write!();
    };
}

macro_rules! write_all_bytes {
    () => {
        deps!();
        # [doc = " Like Write::write_all but returns partial write length even on error"] fn write_all_bytes (mut sink : impl Write , buf : & [u8]) -> (usize , Result < () , Error >) { let mut written = 0 ; while written < buf . len () { match sink . write (& buf [written ..]) { Ok (0) => return (written , Ok (())) , Ok (w) => written += w , Err (e) => return (written , Err (e)) , } } (written , Ok (())) }
    };
}

write_all_bytes!();