macro_rules! deps {
    () => {
        UnhandledPanic!();
        RuntimeFlavor!();
    };
}

macro_rules! FinalConfig {
    () => {
        deps!();
        struct FinalConfig { flavor : RuntimeFlavor , worker_threads : Option < usize > , start_paused : Option < bool > , crate_name : Option < Path > , unhandled_panic : Option < UnhandledPanic > , }
    };
}

FinalConfig!()