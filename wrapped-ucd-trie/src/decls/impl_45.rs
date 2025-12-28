macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl From < Error > for io :: Error { fn from (err : Error) -> io :: Error { io :: Error :: new (io :: ErrorKind :: Other , err) } }
    };
}

impl_45!()