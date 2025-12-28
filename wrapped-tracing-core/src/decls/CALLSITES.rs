macro_rules! deps {
    () => {
        Callsites!();
    };
}

macro_rules! CALLSITES {
    () => {
        deps!();
        static CALLSITES : Callsites = Callsites { list_head : AtomicPtr :: new (ptr :: null_mut ()) , has_locked_callsites : AtomicBool :: new (false) , } ;
    };
}

CALLSITES!()