macro_rules! deps {
    () => {
        DatetimeParseError!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl DatetimeParseError { fn new () -> Self { Self { what : None , expected : None , } } fn what (mut self , what : & 'static str) -> Self { self . what = Some (what) ; self } fn expected (mut self , expected : & 'static str) -> Self { self . expected = Some (expected) ; self } }
    };
}

impl_26!()