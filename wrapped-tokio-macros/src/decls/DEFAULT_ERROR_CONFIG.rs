macro_rules! deps {
    () => {
        RuntimeFlavor!();
        FinalConfig!();
    };
}

macro_rules! DEFAULT_ERROR_CONFIG {
    () => {
        deps!();
        # [doc = " Config used in case of the attribute not being able to build a valid config"] const DEFAULT_ERROR_CONFIG : FinalConfig = FinalConfig { flavor : RuntimeFlavor :: CurrentThread , worker_threads : None , start_paused : None , crate_name : None , unhandled_panic : None , } ;
    };
}

DEFAULT_ERROR_CONFIG!()