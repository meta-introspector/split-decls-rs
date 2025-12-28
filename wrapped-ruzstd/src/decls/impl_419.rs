macro_rules! deps {
    () => {
        ErrorKind!();
    };
}

macro_rules! impl_419 {
    () => {
        deps!();
        impl ErrorKind { fn as_str (& self) -> & 'static str { use ErrorKind :: * ; match * self { Interrupted => "operation interrupted" , UnexpectedEof => "unexpected end of file" , WouldBlock => "operation would block" , Other => "other error" , WriteAllEof => "write_all hit EOF" , } } }
    };
}

impl_419!();