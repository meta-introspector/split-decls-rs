macro_rules! deps {
    () => {
        WorkerThread!();
        Registry!();
        Scope!();
    };
}

macro_rules! do_in_place_scope {
    () => {
        deps!();
        pub (crate) fn do_in_place_scope < 'scope , OP , R > (registry : Option < & Arc < Registry > > , op : OP) -> R where OP : FnOnce (& Scope < 'scope >) -> R , { let thread = unsafe { WorkerThread :: current () . as_ref () } ; let scope = Scope :: < 'scope > :: new (thread , registry) ; scope . base . complete (thread , | | op (& scope)) }
    };
}

do_in_place_scope!()