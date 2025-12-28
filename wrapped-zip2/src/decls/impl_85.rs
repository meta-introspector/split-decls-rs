macro_rules! deps {
    () => {
        ZipFileReader!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl < R : Read > Read for ZipFileReader < '_ , R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { match self { ZipFileReader :: NoReader => invalid_state () , ZipFileReader :: Raw (r) => r . read (buf) , ZipFileReader :: Compressed (r) => r . read (buf) , } } fn read_exact (& mut self , buf : & mut [u8]) -> io :: Result < () > { match self { ZipFileReader :: NoReader => invalid_state () , ZipFileReader :: Raw (r) => r . read_exact (buf) , ZipFileReader :: Compressed (r) => r . read_exact (buf) , } } fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> io :: Result < usize > { match self { ZipFileReader :: NoReader => invalid_state () , ZipFileReader :: Raw (r) => r . read_to_end (buf) , ZipFileReader :: Compressed (r) => r . read_to_end (buf) , } } fn read_to_string (& mut self , buf : & mut String) -> io :: Result < usize > { match self { ZipFileReader :: NoReader => invalid_state () , ZipFileReader :: Raw (r) => r . read_to_string (buf) , ZipFileReader :: Compressed (r) => r . read_to_string (buf) , } } }
    };
}

impl_85!();