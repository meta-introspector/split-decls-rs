macro_rules! deps {
    () => {
        Callsite!();
        Mutex!();
    };
}

macro_rules! LOCKED_CALLSITES {
    () => {
        deps!();
        static LOCKED_CALLSITES : Lazy < Mutex < Vec < & 'static dyn Callsite > > > = Lazy :: new (Default :: default) ;
    };
}

LOCKED_CALLSITES!()