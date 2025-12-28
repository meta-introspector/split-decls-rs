macro_rules! DEFAULT_TEMPDIR {
    () => {
        static DEFAULT_TEMPDIR : OnceLock < PathBuf > = OnceLock :: new () ;
    };
}

DEFAULT_TEMPDIR!();