macro_rules! deps {
    () => {
        StreamWriter!();
    };
}

macro_rules! impl_268 {
    () => {
        deps!();
        impl < W : Write > Seek for StreamWriter < W > { fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { match pos { SeekFrom :: Current (0) | SeekFrom :: End (0) => return Ok (self . bytes_written) , SeekFrom :: Start (x) => { if x == self . bytes_written { return Ok (self . bytes_written) ; } } _ => { } } Err (io :: Error :: new (ErrorKind :: Unsupported , "seek is not supported" ,)) } }
    };
}

impl_268!()