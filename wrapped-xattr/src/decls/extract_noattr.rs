macro_rules! extract_noattr {
    () => {
        pub fn extract_noattr (result : io :: Result < Vec < u8 > >) -> io :: Result < Option < Vec < u8 > > > { result . map (Some) . or_else (| e | { if e . raw_os_error () == Some (crate :: sys :: ENOATTR) { Ok (None) } else { Err (e) } }) }
    };
}

extract_noattr!()