macro_rules! deps {
    () => {
        Runtime!();
    };
}

macro_rules! get {
    () => {
        deps!();
        pub (crate) fn get () -> std :: sync :: MutexGuard < 'static , Runtime > { static RT : std :: sync :: Mutex < Runtime > = std :: sync :: Mutex :: new (Runtime :: new ()) ; RT . lock () . unwrap_or_else (| poisoned | poisoned . into_inner ()) }
    };
}

get!()