macro_rules! deps {
    () => {
        ErrorKind!();
        Error!();
    };
}

macro_rules! Write {
    () => {
        deps!();
        pub trait Write { fn write (& mut self , buf : & [u8]) -> Result < usize , Error > ; fn flush (& mut self) -> Result < () , Error > ; fn write_all (& mut self , mut buf : & [u8]) -> Result < () , Error > { while ! buf . is_empty () { match self . write (buf) { Ok (0) => { return Err (Error :: from (ErrorKind :: WriteAllEof)) ; } Ok (n) => buf = & buf [n ..] , Err (ref e) if e . is_interrupted () => { } Err (e) => return Err (e) , } } Ok (()) } }
    };
}

Write!()