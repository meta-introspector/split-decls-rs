macro_rules! ServiceContext {
    () => {
        # [derive (Debug)] struct ServiceContext (* const c_void) ;
    };
}

ServiceContext!()