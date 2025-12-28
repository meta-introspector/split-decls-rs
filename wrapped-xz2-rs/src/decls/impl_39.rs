macro_rules! deps {
    () => {
        Action!();
        Error!();
        Status!();
        XzDecoder!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < R : BufRead > Read for XzDecoder < R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { loop { let (read , consumed , eof , ret) ; { let input = self . obj . fill_buf () ? ; eof = input . is_empty () ; let before_out = self . data . total_out () ; let before_in = self . data . total_in () ; ret = self . data . process (input , buf , if eof { Action :: Finish } else { Action :: Run }) ; read = (self . data . total_out () - before_out) as usize ; consumed = (self . data . total_in () - before_in) as usize ; } self . obj . consume (consumed) ; let status = ret ? ; if read > 0 || eof || buf . len () == 0 { if read == 0 && status != Status :: StreamEnd && buf . len () > 0 { return Err (io :: Error :: new (io :: ErrorKind :: UnexpectedEof , "premature eof" ,)) ; } return Ok (read) ; } if consumed == 0 { return Err (io :: Error :: new (io :: ErrorKind :: InvalidData , "corrupt xz stream" ,)) ; } } } }
    };
}

impl_39!();