macro_rules! SelfPipeWrite {
    () => {
        trait SelfPipeWrite : Debug + Send + Sync { fn wake_readers (& self) ; }
    };
}

SelfPipeWrite!();