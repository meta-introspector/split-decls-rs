macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! macro_23 {
    () => {
        deps!();
        thread_local ! { static STATE : RefCell < State > = { let config = CONFIG . lock () . unwrap () ; RefCell :: new (State :: new (interest_cache_epoch () , & config)) } ; }
    };
}

macro_23!()