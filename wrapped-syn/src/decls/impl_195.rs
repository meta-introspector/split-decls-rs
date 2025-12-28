macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        impl Extend < Error > for Error { fn extend < T : IntoIterator < Item = Error > > (& mut self , iter : T) { for err in iter { self . combine (err) ; } } }
    };
}

impl_195!();