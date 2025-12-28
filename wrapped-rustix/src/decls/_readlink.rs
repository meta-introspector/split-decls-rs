macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! _readlink {
    () => {
        deps!();
        # [cfg (feature = "alloc")] fn _readlink (path : & CStr , mut buffer : Vec < u8 >) -> io :: Result < CString > { buffer . clear () ; buffer . reserve (SMALL_PATH_BUFFER_SIZE) ; buffer . resize (buffer . capacity () , 0_u8) ; loop { let nread = backend :: fs :: syscalls :: readlink (path , & mut buffer) ? ; let nread = nread as usize ; assert ! (nread <= buffer . len ()) ; if nread < buffer . len () { buffer . resize (nread , 0_u8) ; return Ok (CString :: new (buffer) . unwrap ()) ; } buffer . reserve (1) ; buffer . resize (buffer . capacity () , 0_u8) ; } }
    };
}

_readlink!()