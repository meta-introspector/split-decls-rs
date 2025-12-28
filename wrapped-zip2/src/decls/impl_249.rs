macro_rules! impl_249 {
    () => {
        impl < W : Write + Seek > Write for ZipWriter < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { if ! self . writing_to_file { return Err (io :: Error :: other ("No file has been started")) ; } if buf . is_empty () { return Ok (0) ; } match self . inner . ref_mut () { Some (ref mut w) => { let write_result = w . write (buf) ; if let Ok (count) = write_result { self . stats . update (& buf [0 .. count]) ; if self . stats . bytes_written > spec :: ZIP64_BYTES_THR && ! self . files . last_mut () . unwrap () . 1 . large_file { let _ = self . abort_file () ; return Err (io :: Error :: other ("Large file option has not been set")) ; } } write_result } None => Err (io :: Error :: new (io :: ErrorKind :: BrokenPipe , "write(): ZipWriter was already closed" ,)) , } } fn flush (& mut self) -> io :: Result < () > { match self . inner . ref_mut () { Some (ref mut w) => w . flush () , None => Err (io :: Error :: new (io :: ErrorKind :: BrokenPipe , "flush(): ZipWriter was already closed" ,)) , } } }
    };
}

impl_249!();