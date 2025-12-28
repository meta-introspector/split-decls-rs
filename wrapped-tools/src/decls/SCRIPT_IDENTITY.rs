macro_rules! SCRIPT_IDENTITY {
    () => {
        static SCRIPT_IDENTITY : LazyLock < Mutex < BTreeMap < PathBuf , u32 > > > = LazyLock :: new (| | Mutex :: new (BTreeMap :: new ())) ;
    };
}

SCRIPT_IDENTITY!();