macro_rules! deps {
    () => {
        ServiceContext!();
    };
}

macro_rules! SERVICE_CONTEXT {
    () => {
        deps!();
        static SERVICE_CONTEXT : RwLock < ServiceContext > = RwLock :: new (ServiceContext (std :: ptr :: null ())) ;
    };
}

SERVICE_CONTEXT!();