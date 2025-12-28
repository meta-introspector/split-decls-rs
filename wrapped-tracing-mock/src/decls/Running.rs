macro_rules! deps {
    () => {
        Expect!();
        SpanState!();
    };
}

macro_rules! Running {
    () => {
        deps!();
        struct Running < F : Fn (& Metadata < '_ >) -> bool > { spans : Mutex < HashMap < Id , SpanState > > , expected : Arc < Mutex < VecDeque < Expect > > > , current : Mutex < Vec < Id > > , ids : AtomicUsize , max_level : Option < LevelFilter > , filter : F , name : String , }
    };
}

Running!();